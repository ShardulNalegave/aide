/* eslint-disable @typescript-eslint/no-explicit-any */

import axios from 'axios';
import { useEffect, useState } from "react";

export const useGET = <T>(url: string, defaultVal: T) => {
  const [data, setData] = useState(defaultVal);
  const [error, setError] = useState("");
  const [loaded, setLoaded] = useState(false);

  useEffect(() => {
    (async () => {
      try {
        const res = await axios.get(url);
        setData(res.data ? res.data : defaultVal);
      } catch (err: any) {
        setError(err.message);
      } finally {
        setLoaded(true);
      }
    })();
  }, [url, defaultVal]);

  return { data, error, loaded };
};

export const usePOST = (url: string, payload: unknown) => {
  const [data, setData] = useState(null);
  const [error, setError] = useState("");
  const [loaded, setLoaded] = useState(false);

  useEffect(() => {
    (async () => {
      try {
        const res = await axios.post(url, payload);
        setData(res.data);
      } catch (err: any) {
        setError(err.message);
      } finally {
        setLoaded(true);
      }
    })();
  }, [url, payload]);

  return { data, error, loaded };
};

export const usePUT = (url: string, payload: unknown) => {
  const [data, setData] = useState(null);
  const [error, setError] = useState("");
  const [loaded, setLoaded] = useState(false);

  useEffect(() => {
    (async () => {
      try {
        const res = await axios.put(url, payload);
        setData(res.data);
      } catch (err: any) {
        setError(err.message);
      } finally {
        setLoaded(true);
      }
    })();
  }, [url, payload]);

  return { data, error, loaded };
};

export const useDELETE = (url: string) => {
  const [data, setData] = useState(null);
  const [error, setError] = useState("");
  const [loaded, setLoaded] = useState(false);

  useEffect(() => {
    (async () => {
      try {
        const res = await axios.delete(url);
        setData(res.data);
      } catch (err: any) {
        setError(err.message);
      } finally {
        setLoaded(true);
      }
    })();
  }, [url]);

  return { data, error, loaded };
};