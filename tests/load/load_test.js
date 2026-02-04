import http from 'k6/http';
import { sleep } from 'k6';

let max_vu = `${__ENV.MAX_VU}`;
let url = `${__ENV.URL}`;

export let options = {
  stages: [
    { duration: '5s', target: max_vu / 2 },
    { duration: '10s', target: max_vu },
    { duration: '10m45s', target: max_vu },
    { duration: '15s', target: 0 },
  ],
  thresholds: {
    http_req_duration: ['p(95)<10'], // 95% of requests must finish within 10ms.
    http_req_failed: ['rate<0.01'], // http errors should be less than 1%
  },
  rps: max_vu
};

http.setResponseCallback(
  http.expectedStatuses(404, { min: 200, max: 204 })
);

export default function () {
  let number = Math.floor(Math.random() * 4) + 1;

  if (number == 1) {
    http.get(`${url}/health`);
    sleep(1);
  } else if (number == 2) {
    let data = { url: 'https://hltv.org/' };
    let res = http.post(`${url}/short`, JSON.stringify(data), {
      headers: { 'Content-Type': 'application/json'}
    });
    sleep(1);
    //console.log(`Short: ${res.json().short_url}`);
  } else if (number == 3) {
    let data = { url: 'https://hltv.org/' };
    let res = http.post(`${url}/short`, JSON.stringify(data), {
      headers: { 'Content-Type': 'application/json'}
    });
    let short = res.json().short_url;
    sleep(1);
    let res_get = http.get(`${url}/short/${short}`);
    //console.log(res_get.json());
    sleep(1);
  } else if (number == 4) {
    let short = makeid(6);
    //console.log(short);
    http.get(`${url}/short/${short}`);
    sleep(1);
  } else {
    console.log("What?");
  }
}

function makeid(length) {
  var result           = '';
  var characters       = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789';
  var charactersLength = characters.length;
  for ( var i = 0; i < length; i++ ) {
    result += characters.charAt(Math.floor(Math.random() * charactersLength));
  }
  return result;
}

