extern crate csv;
#[macro_use]
extern crate log;
extern crate loggerv;
extern crate ordered_float;
extern crate rand;
extern crate rayon;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;

use ordered_float::OrderedFloat;
use rand::{Rng, SeedableRng, StdRng};
use rayon::prelude::*;
use std::cmp;
use std::collections::HashSet;
use std::path::Path;
use std::path::PathBuf;
use structopt::StructOpt;

type DistanceMatrix = Vec<Vec<f64>>;
type Route = Vec<usize>;

#[derive(Deserialize, Clone)]
struct City {
    x: f64,
    y: f64,
}

struct TSP {
    cities: Vec<City>,
    strategy: Strategy,
}

#[derive(StructOpt, Debug)]
enum Strategy {
    #[structopt(name = "greedy")]
    Greedy,
    #[structopt(name = "dp")]
    DP,
    #[structopt(name = "sa")]
    SA,
    #[structopt(name = "twoopt")]
    TwoOpt {
        #[structopt(short = "g", long = "goal")]
        goal: Option<f64>,
    },
    #[structopt(name = "mapreduce")]
    MapReduce,
}

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(short = "v", parse(from_occurrences))]
    verbose: u64,
    #[structopt(parse(from_os_str))]
    input: PathBuf,
    #[structopt(subcommand)]
    strategy: Strategy,
}

fn main() {
    let opt = Opt::from_args();
    loggerv::init_with_verbosity(opt.verbose).unwrap();
    let cities = TSP::read_from_csv(opt.input);
    let tsp = TSP::new(cities, opt.strategy);
    let route = tsp.solve();
    debug_assert_eq!(
        route.iter().cloned().collect::<HashSet<usize>>().len(),
        tsp.cities.len()
    );
    println!("index");
    for i in &route {
        println!("{}", i);
    }
    info!("Distance: {}", tsp.route_dist(&route));
}

impl TSP {
    fn read_from_csv<P: AsRef<Path>>(input: P) -> Vec<City> {
        let mut reader = csv::Reader::from_path(input).unwrap();
        reader.deserialize().map(|r| r.unwrap()).collect()
    }

    fn new(cities: Vec<City>, strategy: Strategy) -> TSP {
        TSP { cities, strategy }
    }

    fn distance_matrix(&self) -> DistanceMatrix {
        let n = self.cities.len();
        let mut matrix = vec![vec![0.0; n]; n];
        for i in 0..n {
            for j in 0..i {
                matrix[i][j] = self.dist(i, j);
                matrix[j][i] = matrix[i][j];
            }
        }
        matrix
    }

    fn dist(&self, i: usize, j: usize) -> f64 {
        (self.cities[i].x - self.cities[j].x).hypot(self.cities[i].y - self.cities[j].y)
    }

    fn route_dist(&self, route: &Route) -> f64 {
        route.iter().fold(0.0, |acc, &i| {
            acc + self.dist(route[i], route[(i + 1) % self.cities.len()])
        })
    }

    fn solve(&self) -> Route {
        match self.strategy {
            Strategy::Greedy => self.greedy(),
            Strategy::DP => self.dp(),
            Strategy::SA => self.sa(),
            Strategy::TwoOpt { goal } => self.two_opt(goal),
            Strategy::MapReduce => self.map_reduce(),
        }
    }

    fn greedy(&self) -> Route {
        let mut unvisited = (1..self.cities.len()).collect::<HashSet<usize>>();
        let mut current = 0;
        let mut route = vec![current];
        while let Some(&i) = unvisited
            .iter()
            .min_by_key(|&&i| OrderedFloat(self.dist(current, i)))
        {
            route.push(i);
            unvisited.remove(&i);
            current = i;
        }
        route
    }

    fn dp(&self) -> Route {
        let dist = self.distance_matrix();
        let n = self.cities.len();
        let mut dp = vec![vec![std::f64::MAX; 1 << n]; n];
        let mut backtrack = vec![vec![0; 1 << n]; n];

        for i in 0..n {
            dp[i][0] = dist[i][0];
        }

        for unvisited in 0..(1 << n) {
            for current in 0..n {
                if (unvisited & (1 << current)) != 0 {
                    continue;
                }
                for next in 0..n {
                    if (unvisited & (1 << next)) == 0 {
                        continue;
                    }
                    let next_unvisited = unvisited ^ (1 << next);
                    let score = dp[next][next_unvisited] + dist[current][next];
                    if score < dp[current][unvisited] {
                        dp[current][unvisited] = score;
                        backtrack[current][unvisited] = next;
                    }
                }
            }
        }
        let mut current = 0;
        let mut unvisited = (1 << n) - 1;
        unvisited ^= 1 << current;
        debug!("Distance (dp): {}", dp[current][unvisited]);
        let mut route = vec![current];
        while unvisited != 0 {
            current = backtrack[current][unvisited];
            route.push(current);
            unvisited ^= 1 << current;
        }
        route
    }

    fn sa(&self) -> Route {
        fn next_route<R: Rng>(
            n: usize,
            dist: &DistanceMatrix,
            route: &Route,
            rng: &mut R,
        ) -> (f64, usize, usize) {
            let i = rng.gen_range(0, n);
            let j = rng.gen_range(0, n);
            let (i, j) = (cmp::min(i, j), cmp::max(i, j));
            if i + 1 < j {
                let i2 = if i == 0 { n - 1 } else { i - 1 };
                let j2 = j - 1;
                let dist_add = dist[route[i2]][route[j2]] + dist[route[i]][route[j]];
                let dist_sub = dist[route[i2]][route[i]] + dist[route[j2]][route[j]];
                let dist_diff = dist_add - dist_sub;
                (dist_diff, i, j)
            } else {
                next_route(n, dist, route, rng)
            }
        }

        let dist = self.distance_matrix();
        let mut rng: StdRng = SeedableRng::from_seed([0; 32]);

        let n = self.cities.len();
        let mut route = (0..n).collect::<Route>();
        let mut iteration = 0;
        let mut temperature = 100000.0;
        let cooling_rate = 0.9999999;
        let absolute_temperature = 0.000001;

        while temperature > absolute_temperature {
            if iteration % 1_000_000 == 0 {
                debug!(
                    "iteration: {}, temperature: {:10.8}, route_dist: {}",
                    iteration,
                    temperature,
                    self.route_dist(&route)
                );
            }
            let (delta, i, j) = next_route(n, &dist, &route, &mut rng);
            if delta < 0.0 || (-delta / temperature).exp() > rng.gen::<f64>() {
                route[i..j].reverse();
            }
            temperature *= cooling_rate;
            iteration += 1;
        }
        route
    }

    fn two_opt(&self, goal: Option<f64>) -> Route {
        let dist = self.distance_matrix();
        let mut route = self.greedy();
        let n = self.cities.len();
        let mut total_dist = self.route_dist(&route);
        loop {
            let mut improved = false;
            for i in 0..n {
                for j in (i + 2)..n {
                    let i2 = if i == 0 { n - 1 } else { i - 1 };
                    let j2 = j - 1;
                    let dist_add = dist[route[i2]][route[j2]] + dist[route[i]][route[j]];
                    let dist_sub = dist[route[i2]][route[i]] + dist[route[j2]][route[j]];
                    let dist_diff = dist_add - dist_sub;
                    if dist_diff < 0.0 {
                        improved = true;
                        route[i..j].reverse();
                        if let Some(goal) = goal {
                            total_dist += dist_diff;
                            if total_dist < goal {
                                info!("Found: goal:{}, total: {}", goal, total_dist);
                                return route;
                            }
                        }
                    }
                }
            }
            if !improved {
                if let Some(goal) = goal {
                    warn!("Failed: goal {} is not satisfied", goal);
                }
                return route;
            }
        }
    }

    fn map_reduce(&self) -> Route {
        #[derive(Clone)]
        struct Block {
            cities: Vec<City>,
            city_indices: Vec<usize>,
        }

        impl Block {
            fn new() -> Block {
                Block {
                    cities: Vec::new(),
                    city_indices: Vec::new(),
                }
            }
        }

        // TODO(hayato): Avoid magic numbers
        let dx = 1600;
        let dy = 900;
        let block_size = 25;
        assert_eq!(dx % block_size, 0);
        assert_eq!(dy % block_size, 0);
        let bx = dx / block_size;
        let by = dy / block_size;
        let blocks_num = bx * by;
        let mut blocks = vec![Block::new(); blocks_num];
        for (i, city) in self.cities.iter().enumerate() {
            let x = (city.x as usize) / block_size;
            let y = (city.y as usize) / block_size;
            let block_index = y * bx + x;
            blocks[block_index].city_indices.push(i);
            blocks[block_index].cities.push(city.clone());
        }

        let mut routes = blocks
            .into_par_iter()
            // .into_iter()
            .map(|Block {
                 cities,
                 city_indices,
             }| {
                TSP::new(cities, Strategy::Greedy)
                    .solve()
                    .into_iter()
                    .map(|i| city_indices[i])
                    .collect::<Route>()
            })
            .collect::<Vec<_>>();

        let mut route = Vec::with_capacity(self.cities.len());

        // TODO(hayato): Use hilbert curve
        assert_eq!(bx % 2, 0);
        assert_eq!(by % 2, 0);
        let hx = bx / 2;
        for y in 0..by {
            if y % 2 == 0 {
                for x in (0..hx).rev() {
                    route.append(&mut routes[y * bx + x]);
                }
            } else {
                for x in 0..hx {
                    route.append(&mut routes[y * bx + x]);
                }
            }
        }
        for y in (0..by).rev() {
            if y % 2 == 0 {
                for x in (hx..bx).rev() {
                    route.append(&mut routes[y * bx + x]);
                }
            } else {
                for x in hx..bx {
                    route.append(&mut routes[y * bx + x]);
                }
            }
        }
        route
    }
}