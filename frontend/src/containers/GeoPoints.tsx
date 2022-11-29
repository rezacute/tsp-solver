import React, { useEffect, useState } from "react";
import Table from "@mui/material/Table";
import TableBody from "@mui/material/TableBody";
import TableCell from "@mui/material/TableCell";
import TableContainer from "@mui/material/TableContainer";
import TableHead from "@mui/material/TableHead";
import TableRow from "@mui/material/TableRow";
import Paper from "@mui/material/Paper";
import Box from "@mui/material/Box";
import Container from "@mui/material/Container";
import Chip from "@mui/material/Chip";
import Stack from "@mui/material/Stack";

const GeoPointAPI = {
  get: async () => await (await fetch(`/api/tsp`)).json(),
  getResult: async () => await (await fetch(`/api/tsp/solve`)).json(),
};

const ShowRoute = (props: {
  points: Array<GeoPoint>;
  route: Array<number>;
}) => {
  let ln = props.points.length;
  let rs: Array<string> = [];
  for (let i = 0; i < props.route.length; i++) {
    let rt = props.route[i];
    rs.push(props.points[rt].label);
  }
  return (
    <>
      {rs.map((n, index) => {
        <span>[{n}]</span>;
      })}
      <span>[Start/Finish]</span>
    </>
  );
};

export const GeoPoints = () => {
  const [geoPoints, setGeoPoints] = useState<Array<GeoPoint>>();
  const [solvedPoints, setSolvedPoints] = useState<Array<GeoPoint>>();
  const [processing, setProcessing] = useState<boolean>(false);
  const [tspResult, setTspResult] = useState<TSPResult>();
  const solveTSP = async (points: Array<GeoPoint>) => {
    setProcessing(true);
    console.log("->", points.length);

    GeoPointAPI.getResult().then((result: TSPResult) => {
      setTspResult(result);
      let solvedPoints: Array<GeoPoint> = [];
      result.route.map((rt) => {
        solvedPoints.push(points[rt]);
      });
      setSolvedPoints(solvedPoints);
      console.log(result.distance);
      setProcessing(false);
    });
  };
  useEffect(() => {
    setProcessing(true);
    GeoPointAPI.get().then((geopoints) => {
      setGeoPoints(geopoints);

      setProcessing(false);
    });
  }, []);
  return (
    <div style={{ display: "flex" }}>
      <div style={{ flexFlow: "column", textAlign: "left", flex: "50%", marginTop:10 }}>
        <TableContainer sx={{ maxHeight: 440 }} component={Paper}>
          <Table sx={{ minWidth: 650 }} aria-label="simple table" stickyHeader>
            <TableHead>
              <TableRow>
                <TableCell>#</TableCell>
                <TableCell align="right">Point</TableCell>
                <TableCell align="right">latitude</TableCell>
                <TableCell align="right">longitude</TableCell>
              </TableRow>
            </TableHead>
            <TableBody>
              {geoPoints?.map((point, index) => (
                <TableRow
                  sx={{ "&:last-child td, &:last-child th": { border: 0 } }}
                >
                  <TableCell component="th" scope="row">
                    {index}
                  </TableCell>
                  <TableCell align="right">{point.label}</TableCell>
                  <TableCell align="right">{point.lat}</TableCell>
                  <TableCell align="right">{point.lng}</TableCell>
                </TableRow>
              ))}
            </TableBody>
          </Table>
        </TableContainer>
      </div>
      <div
        style={{
          display: "flex",
          flexFlow: "column",
          textAlign: "left",
          flex: "50%",
        }}
      >
        <div className="Form" style={{marginLeft:20}}>
          <span>
          <button
              disabled={processing}
              style={{ height: "40px" }}
              onClick={() => solveTSP(geoPoints || [])}
            >
              Solve
            </button>
          </span>
        </div>
        <span style={{marginLeft:20}}>result:</span>
        <hr></hr>

        {tspResult != undefined ? (
          <Container maxWidth="sm">
            <div>
                <span>
                  distance: {tspResult.distance} meters
                </span>
              </div>
              <hr></hr>
              <span>route:</span>
              <Stack direction="row" spacing={1} maxWidth="500px" flexWrap="wrap" >
                {geoPoints?.map((point, index) => (
                  <Chip label={point.label} size="small" variant="outlined" />
                ))}
              </Stack>

              
            
          </Container>
        ) : (
          <div></div>
        )}
        
      </div>
    </div>
  );
};
