import http from 'k6/http';
import { check } from 'k6';
import { SharedArray } from 'k6/data';
import { randomItem } from 'https://jslib.k6.io/k6-utils/1.2.0/index.js';

const BASE_URL = 'http://localhost:3000/api/payment/orderbyid';

const orderIds = new SharedArray('orderIds', function () {
    return [
        "78e744e6-89c8-44ef-90c5-273db925382a",
        "13a1f350-d0c8-4c1a-9b5d-e284560ba464",
        "83af5f27-ee95-4ccc-97ec-9144df44237e",
        "7c54eef8-8c8d-4960-9041-b325b3607a87",
        "776c1ff1-3dc4-40a7-aa3a-873f0bbc4f80",
        "831327f5-49f0-463b-9715-052df2d5fe89",
        "f6ac139b-c21e-4c49-8549-b03b6751db53",
        "2003ce9d-4aa0-47b2-bc35-43db9780a4b5",
        "c4e3fb38-773f-4682-80c8-b550c1b68578",
        "5cc8b355-4013-4cfc-ba5d-c5ccec334897",
        "93be299f-ca28-4b00-a0a5-3676023de231",
        "2c56df38-bffe-4504-beaf-111a05c6c1c7",
        "ffb0c906-550b-4e3c-ab63-0bd6d84d5049",
        "062abd90-e237-4c87-94d3-8e64cbbd3d5c",
        "e0d750f9-5c4a-49dc-a52c-c483bed84759",
        "5432e818-bd31-4691-8817-b1aa1e2687d8"
    ];
});

export const options = {
    scenarios: {
        steady_load_test: {
            executor: 'constant-arrival-rate',
            rate: 150,               // 50 requests per second
            timeUnit: '1s',         // Each second
            duration: '2h',         // Run for 2 hours
            preAllocatedVUs: 100,   // Start with 100 VUs
            maxVUs: 500,            // Allow up to 500 if needed
        },
    },
    thresholds: {
        http_req_duration: ['p(95)<500'], // 95% of requests should be under 500ms
        http_req_failed: ['rate<0.01'],   // Less than 1% should fail
    }
};

export default function () {
    const randomOrderId = randomItem(orderIds);
    const url = `${BASE_URL}?order_id=${randomOrderId}`;

    const headers = {
        'Authorization': 'Bearer eyJhbGciOiJSUzI1NiIsInR5cCIgOiAiSldUIiwia2lkIiA6ICJ4ZVJYM2FYdjF4dEltVldGX3dWcGhBWHNJazA1cmlXUkljVjNyckxkcUZNIn0.eyJleHAiOjE3MzczMTc2OTQsImlhdCI6MTczNzMwNjg5NCwianRpIjoiNTAxMzU0MWYtMTI2Yy00NTM5LTk1MjQtMTc1ZjJiNTk5YzhiIiwiaXNzIjoiaHR0cDovL2xvY2FsaG9zdDozMDAwL2F1dGgvcmVhbG1zL3Nob3AiLCJhdWQiOiJhY2NvdW50Iiwic3ViIjoiMDQwOTA0ODEtYjE5Yy00MWU5LWJhMWQtZjQ0NWMzMzE0ODI5IiwidHlwIjoiQmVhcmVyIiwiYXpwIjoiZnJvbnRlbmQtY2xpZW50Iiwic2lkIjoiYTBlOGE0ZjgtZTc1Yi00NmQ3LTk0ZTEtMDU1ZmJlNDM0MDRmIiwiYWNyIjoiMSIsImFsbG93ZWQtb3JpZ2lucyI6WyJodHRwOi8vbG9jYWxob3N0OjMwMDAiXSwicmVhbG1fYWNjZXNzIjp7InJvbGVzIjpbIm9mZmxpbmVfYWNjZXNzIiwiZGVmYXVsdC1yb2xlcy1zaG9wIiwidW1hX2F1dGhvcml6YXRpb24iLCJ1c2VyIl19LCJyZXNvdXJjZV9hY2Nlc3MiOnsiYWNjb3VudCI6eyJyb2xlcyI6WyJtYW5hZ2UtYWNjb3VudCIsIm1hbmFnZS1hY2NvdW50LWxpbmtzIiwidmlldy1wcm9maWxlIl19LCJmcm9udGVuZC1jbGllbnQiOnsicm9sZXMiOlsiY2xpZW50X3VzZXIiXX19LCJzY29wZSI6InByb2ZpbGUgZW1haWwiLCJlbWFpbF92ZXJpZmllZCI6ZmFsc2UsIm5hbWUiOiJ1c2VyIHVzZXIiLCJwcmVmZXJyZWRfdXNlcm5hbWUiOiJ1c2VyIiwiZ2l2ZW5fbmFtZSI6InVzZXIiLCJmYW1pbHlfbmFtZSI6InVzZXIiLCJlbWFpbCI6InVzZXJAdXNlci5kZSJ9.RVOR1BlUvE0XTZhtqUOp6c1WYPiiOAtmNpcUINTNgYcomk57PRxdFhoSa2YJYUhxPFNwpa0Tw7HQD5w7tSODMUtj3qOI6uNE8Va7RNNEZuCo-PWwQq2cPxYJ-T-RWz9jg-yzURka8cjTv4LT-iORjSPZXYOL6Ydf9wQ--xZs9K0-9ilLNaU_oUKiiC5k_lCrIJVLsH4PDRvtEtW4stawy9ceTsOl5AswzShs82tX0EawERKgAlbeJiFCTUxJDWG1IlPzgbUUEJltWdh2DfNkufkB2daJ2YJPH-L5LlfzrhwVO584V6eRAT7iIHfys1RxOECqr2QRlWmazb_y8-LW2A',
        'Content-Type': 'application/json'
    };

    const res = http.get(url, { headers });

    check(res, {
        'is status 200': (r) => r.status === 200,
        'response time < 500ms': (r) => r.timings.duration < 500
    });
}
