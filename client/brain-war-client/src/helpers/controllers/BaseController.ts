import axios from "axios";

class GameController {
  get(url: string, config: object = {}) {
    return new Promise((resolve, reject) => {
      axios
        .get(url, config)
        .then((res) => {
          resolve(res.data);
        })
        .catch((err) => {
          console.warn(err);
          resolve(err.response.data);
          // reject(err.response.data)
        });
    });
  }

  post(url: string, data: any = null, config: object = {}) {
    return new Promise((resolve, reject) => {
      axios
        .post(url, data, config)
        .then((res) => {
          resolve(res.data);
        })
        .catch((err) => {
          console.warn(err);
          resolve(err.response.data);
          // reject(err.response.data)
        });
    });
  }

  put(url: string, data: any = null, config: object = {}) {
    return new Promise((resolve, reject) => {
      axios
        .put(url, data, config)
        .then((res) => {
          resolve(res.data);
        })
        .catch((err) => {
          console.warn(err);
          resolve(err.response.data);
          // reject(err.response.data)
        });
    });
  }

  delete(url: string, config: object = {}) {
    return new Promise((resolve, reject) => {
      axios
        .delete(url, config)
        .then((res) => {
          console.log("delete", res.data);
          resolve(res.data);
        })
        .catch((err) => {
          console.warn(err);
          resolve(err.response.data);
          // reject(err.response.data)
        });
    });
  }
}

export default new GameController();
