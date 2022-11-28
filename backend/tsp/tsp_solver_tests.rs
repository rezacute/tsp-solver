mod tsp_solver_tests{
    

    use super::super::{*};

    #[test]
    fn load_sample_file_test(){
        
        println!("{:?}",&get_dataset().lock().unwrap().points)
    }
}