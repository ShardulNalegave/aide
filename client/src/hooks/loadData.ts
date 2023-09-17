/* eslint-disable @typescript-eslint/no-explicit-any */

import axios from 'axios';
import { useEffect, useState } from "react";

export const useGET = <T>(url: string, defaultVal: T) => {
  const [data, setData] = useState(defaultVal);
  const [error, setError] = useState("");
  const [loaded, setLoaded] = useState(false);
  const [shouldReload, setShouldReload] = useState(true);

  useEffect(() => {
    const get = async () => {
      try {
        const res = await axios.get(url);
        setData(res.data ? res.data : defaultVal);
      } catch (err: any) {
        setError(err.message);
      } finally {
        setLoaded(true);
      }
    };
    if (shouldReload) {
      get();
      setShouldReload(false);
    }
  }, [url, defaultVal, shouldReload]);

  const reload = () => setShouldReload(true);

  return { data, error, loaded, reload };
};