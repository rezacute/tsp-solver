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
import ReactLeafletDriftMarker from "react-leaflet-drift-marker"
import {
  MapContainer,
  TileLayer,
  useMap,
  Marker,
  CircleMarker,
  Popup,
  Polyline,
} from "react-leaflet";
import Button from "@mui/material/Button";
import Typography from "@mui/material/Typography";
import Modal from "@mui/material/Modal";
import "leaflet/dist/leaflet.css";
const style = {
  position: "absolute" as "absolute",
  top: "50%",
  left: "50%",
  transform: "translate(-50%, -50%)",
  width: 1000,
  bgcolor: "background.paper",
  border: "2px solid #000",
  boxShadow: 24,
  p: 4,
};
const GeoPointAPI = {
  get: async () => await (await fetch(`/api/tsp`)).json(),
  getResult: async () => await (await fetch(`/api/tsp/solve`)).json(),
};


interface Latlong {
  lat:number,
  lng:number
}

export const GeoPoints = () => {
  const [open, setOpen] = React.useState(false);
  const [pos,setPos] = React.useState<Latlong>({lat:48.0697249,lng:7.3602374});
  const [idx,setIdx] = React.useState<number>(0);
  const handleOpen = () => setOpen(true);
  const handleClose = () => setOpen(false);
  const [geoPoints, setGeoPoints] = useState<Array<GeoPoint>>();
  const [solvedPoints, setSolvedPoints] = useState<Array<GeoPoint>>();
  const [processing, setProcessing] = useState<boolean>(false);
  const [tspResult, setTspResult] = useState<TSPResult>();
  const limeOptions = { color: "lime" };
  const redOptions = { color: 'red' }

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
  useEffect(()=>{
    setTimeout(() => {// updates position every 5 sec
      console.log("tick");
      if(open){
        
        
        if(solvedPoints!.length>idx){
          setIdx(idx+1);
        }else{
          setIdx(0);
          setPos({lat:solvedPoints![0].lat,lng:solvedPoints![0].lng})
          return;
        }
        setPos({lat:solvedPoints![idx].lat,lng:solvedPoints![idx].lng})
      }
  }, 3000);
  })
  return (
    <div style={{ display: "flex" }}>
      <div
        style={{
          flexFlow: "column",
          textAlign: "left",
          flex: "50%",
          marginTop: 10,
        }}
      >
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
        <div className="Form" style={{ marginLeft: 20 }}>
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
        <span style={{ marginLeft: 20 }}>result:</span>
        <hr></hr>

        {tspResult != undefined ? (
          <Container maxWidth="sm">
            <div>
              <div>
                <Modal
                  open={open}
                  onClose={handleClose}
                  aria-labelledby="modal-modal-title"
                  aria-describedby="modal-modal-description"
                >
                  <Box sx={style}>
                    <MapContainer
                      center={[48.0621758, 7.3561662]}
                      zoom={16}
                      scrollWheelZoom={false}
                      style={{ height: "90vh", width: "100wh" }}
                    >
                      <TileLayer
                        attribution='&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors'
                        url="https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png"
                      />
                      {solvedPoints?.map((point, index) => (
                        <>
                        
                          {index > 0 ? (
                            <>
                              <Polyline
                                pathOptions={limeOptions}
                                positions={[
                                  [
                                    solvedPoints[index - 1].lat,
                                    solvedPoints[index - 1].lng,
                                  ],
                                  [point.lat, point.lng],
                                ]}
                              />
                            </>
                          ) : (
                            <>
                              <Polyline
                                pathOptions={limeOptions}
                                positions={[
                                  [
                                    solvedPoints[solvedPoints.length - 1].lat,
                                    solvedPoints[solvedPoints.length - 1].lng,
                                  ],
                                  [point.lat, point.lng],
                                ]}
                              />
                            </>
                          )
                          }
                          <CircleMarker center={[point.lat, point.lng]} pathOptions={redOptions} radius={2}>
      <Popup>{point.label}</Popup>
    </CircleMarker>
                        </>
                      ))}
                      <ReactLeafletDriftMarker
            // if position changes, marker will drift its way to new position
            position={pos}
            // time in ms that marker will take to reach its destination
            duration={1000}
            
             >
            <Popup>{solvedPoints![idx].label}</Popup>
      
        </ReactLeafletDriftMarker>
                    </MapContainer>
                  </Box>
                </Modal>
              </div>
              <span>distance: {tspResult.distance} meters</span>
            </div>
            <hr></hr>
            <Stack direction="row" spacing={1} maxWidth="500px" flexWrap="wrap">
              <span>route:</span>
              <Button onClick={handleOpen}>Open Map</Button>
            </Stack>

            <Stack direction="row" spacing={1} maxWidth="500px" flexWrap="wrap">
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
