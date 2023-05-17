import axios from 'axios';
import * as actions from '../baseapi';

const BASEAPI = "http://127.0.0.1:8000/";

export const serializeParams = (params: any) => {
  if (!params) {
    return '';
  }

  let serialized = Object.keys(params)
    .map(key => {
      const value = params[key];
      if (Array.isArray(value)) {
        return Object.keys(value)
          .map((idx: any) => {
            return (
              encodeURIComponent(key) + '=' + encodeURIComponent(value[idx])
            );
          })
          .join('&');
      } else {
        return encodeURIComponent(key) + '=' + encodeURIComponent(value);
      }
    })
    .join('&');

  return `${serialized}`;
};

const api =
  ({dispatch}: any) =>
  (next: any) =>
  async (action: { type: string; payload: { data: any; params: any; url?: any; customUrl?: any; extraData?: any; method?: any; onStart?: any; onSuccess?: any; onError?: any; headers?: any; files?: any; hideErrors?: any; hideToastErrors?: any; }; }) => {
    if (action.type !== actions.apiCallBegan.type) return next(action);

    const {
      method,
      data,
      params,
      onStart,
      onSuccess,
      onError,
      headers,
      files,
      hideErrors,
      hideToastErrors,
    } = action.payload;
    if (onStart)
      dispatch({
        type: onStart,
        payload: action.payload.data || action.payload.params,
      });

    next(action);
    try {
      var formData = data;
      var requestHeaders = headers;
      if (files) {
        requestHeaders = {
          ...headers,
          ...{'Content-Type': 'multipart/form-data'},
        };
        formData = new FormData();
        formData.append('parameters', JSON.stringify(data));

        files.forEach((file: { size?: any; }, i: string) => {
          if (file) {
            // if file is empty, send null instead
            let fileToAdd =
              Object.keys(file).length > 0 || file.size > 0 ? file : null;
            formData.append('file' + i, fileToAdd);
          }
        });
      }

      let url = action.payload.url;
      let customUrl = action.payload.customUrl;
      let baseApiUrl = BASEAPI;

      if (customUrl) {
        baseApiUrl = '';
        url = customUrl;
      }

      const response = await axios.request({
        baseURL: baseApiUrl,
        url,
        method,
        data: formData,
        params,
        headers: requestHeaders,
        paramsSerializer: {
          serialize: function (params) {
            return serializeParams(params);
          },
        },
      });

      // General
      dispatch(actions.apiCallSucess(response.data));
      // Specific
      if (onSuccess) {
        dispatch({
          type: onSuccess,
          payload: response.data,
          args: action.payload.data || action.payload.params || false,
          extraData: action.payload.extraData,
        });
      }
    } catch (error) {
      console.log(error);
      if (!hideErrors) {
        if (hideToastErrors === undefined) {
          dispatch(actions.apiCallFailed("Error"));
        }
      }

      if (onError) dispatch({type: onError, payload: "error"});
    }
  };

export default api;
