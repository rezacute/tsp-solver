import React, { useEffect, useState } from 'react'

const GeoPointAPI = {
  get: async () =>
      await (await fetch(`/api/tsp`)).json(),
}

export const GeoPoints = () => {
  
  const [GeoPoints, setGeoPoints] = useState<Array<GeoPoint>>()
  useEffect(() => {
    //setProcessing(true)
    GeoPointAPI.get().then((geopoints) => {
      setGeoPoints(geopoints)
      //setProcessing(false)
    })
  }, [])
  return (
      <div style={{ display: 'flex', flexFlow: 'column', textAlign: 'left' }}>
        <h1>GeoPoints</h1>
        
        {GeoPoints?.map((GeoPoint, index) =>
            (
                <div className="Form">
                  <div style={{ flex: 1 }}>
                    #{index} . {GeoPoint.label} ({GeoPoint.lat},{GeoPoint.lng})
                  </div>
                  
                </div>
            )
        )}
        
        
      </div>
  )
}
