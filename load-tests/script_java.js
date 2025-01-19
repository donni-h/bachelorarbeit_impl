import http from 'k6/http';
import { check, sleep } from 'k6';
import { SharedArray } from 'k6/data';
import { randomItem } from 'https://jslib.k6.io/k6-utils/1.2.0/index.js';

const BASE_URL = 'http://localhost:3000/api/payment/orderbyid'; // Change to your actual API

const orderIds = new SharedArray('orderIds', function () {
    return [
        "e0c18b2c-74ea-423d-b3c8-c7259f42610f",
        "a5984844-f017-42f9-8b42-b66c2fcbb7c9",
        "97d74c52-81d2-4518-a662-5a2636401ef1",
        "32081d05-2958-4803-b08a-9bbbe088fba1",
        "51570cf6-0547-4061-bc25-acab9a91b3af",
        "797e9ffa-e513-42f0-b20b-38f321ded303",
        "27e76fba-5d6b-4eff-9b8e-1e37616f6589",
        "1d41ceda-0a2e-4f30-ba83-feb9e6bb64f6",
        "05d0876a-8819-49d6-a3b8-c89b48ec7960",
        "68aeb1c9-feea-471a-9c29-ebf208c66e5a",
        "3075533c-6cf2-478b-baf2-bc1c72589cc7",
        "7010d0e7-63a9-4300-a30a-f36d9103964a",
        "fd3cc674-f167-4c1f-b029-31a05d59a916",
        "4b1387e5-8282-4647-af67-1aeeb8587b5c",
        "70e8c47c-cc68-4994-9474-f296c3442842",
        "3bedca21-712a-47c5-a085-84d618d7fe2e",
        "bdac0203-ccdf-41f7-a8fc-1d72734a8a9b",
        "b94c2da3-21e5-4463-a828-217910f59232",
        "c9f34569-48ce-4148-a138-2bc5ce5a13cb",
        "9657d4c0-c902-4633-ae0f-5f5df103517d",
        "caa9754b-cf19-4d5b-8a09-edf5708d562d",
        "1aff2c48-bf8a-4c8a-903b-998ff2f5f3d0",
        "f86ebbf4-5ef3-44bb-ad75-e04815665967",
        "744004b1-61ad-4a99-8f64-8265b69a3a96",
        "f47a923c-bed3-4c84-bff6-75767c5f5c6f",
        "967af49a-6258-4cb4-81ff-6c70d069e786",
        "6c14d6d5-3b05-487f-93e5-1a4fb35f8581",
        "4b55b609-3657-4398-a5bf-3b7023a7610d",
        "cebaa6a8-8379-4a39-9321-350350678a25"
    ]
});

export const options = {
    stages: [
        { duration: '30s', target: 200 },  // Schnell auf 200 gleichzeitige User
        { duration: '30s', target: 500 },  // Dann auf 500 User steigern
        { duration: '1m', target: 1000 },  // 1 Minute lang 1000 User halten
        { duration: '30s', target: 500 },  // Langsam wieder runterfahren
        { duration: '30s', target: 0 },    // Test beenden
    ],
    thresholds: {
        http_req_duration: ['p(95)<500'], // 95% of requests must be below 500ms
        http_req_failed: ['rate<0.01'],   // Less than 1% should fail
    }
};

export default function () {
    const randomOrderId = randomItem(orderIds);
    const url = `${BASE_URL}?order_id=${randomOrderId}`;

    const headers = {
        'Authorization': 'Bearer eyJhbGciOiJSUzI1NiIsInR5cCIgOiAiSldUIiwia2lkIiA6ICJ4ZVJYM2FYdjF4dEltVldGX3dWcGhBWHNJazA1cmlXUkljVjNyckxkcUZNIn0.eyJleHAiOjE3MzczMjYxMTcsImlhdCI6MTczNzMxNTMxNywianRpIjoiMmU0MDdiMTgtOWYzOC00Mzc1LThlN2ItYTMwNDJmMjRlYTRmIiwiaXNzIjoiaHR0cDovL2xvY2FsaG9zdDozMDAwL2F1dGgvcmVhbG1zL3Nob3AiLCJhdWQiOiJhY2NvdW50Iiwic3ViIjoiMDQwOTA0ODEtYjE5Yy00MWU5LWJhMWQtZjQ0NWMzMzE0ODI5IiwidHlwIjoiQmVhcmVyIiwiYXpwIjoiZnJvbnRlbmQtY2xpZW50Iiwic2lkIjoiYzlkN2NhY2ItYWMwNS00MDg5LThlOWItMGY4NDRkYWQ4MzEyIiwiYWNyIjoiMSIsImFsbG93ZWQtb3JpZ2lucyI6WyJodHRwOi8vbG9jYWxob3N0OjMwMDAiXSwicmVhbG1fYWNjZXNzIjp7InJvbGVzIjpbIm9mZmxpbmVfYWNjZXNzIiwiZGVmYXVsdC1yb2xlcy1zaG9wIiwidW1hX2F1dGhvcml6YXRpb24iLCJ1c2VyIl19LCJyZXNvdXJjZV9hY2Nlc3MiOnsiYWNjb3VudCI6eyJyb2xlcyI6WyJtYW5hZ2UtYWNjb3VudCIsIm1hbmFnZS1hY2NvdW50LWxpbmtzIiwidmlldy1wcm9maWxlIl19LCJmcm9udGVuZC1jbGllbnQiOnsicm9sZXMiOlsiY2xpZW50X3VzZXIiXX19LCJzY29wZSI6InByb2ZpbGUgZW1haWwiLCJlbWFpbF92ZXJpZmllZCI6ZmFsc2UsIm5hbWUiOiJ1c2VyIHVzZXIiLCJwcmVmZXJyZWRfdXNlcm5hbWUiOiJ1c2VyIiwiZ2l2ZW5fbmFtZSI6InVzZXIiLCJmYW1pbHlfbmFtZSI6InVzZXIiLCJlbWFpbCI6InVzZXJAdXNlci5kZSJ9.YDahUoBPl0Jnzl73tL3-Pv3DijNjmk0hmOz5V_ot2lz0sXer-QLy8MhWLL8iQT8FeZgN3nf6jb1oY2YCvjZ4xsPsd0OIM6r6Rv25vfarneJETVx_ERruSDGKrLvzEixFnOHyBSGQWFrpcz3VhidpnX7CxAsMGT6DBNyii_9RtKWL9lVCdnJqye4Iy39KTjBzBuvLFCZ4HDbTm6YWPyNu3g-Pnng-HwgCjNjc9wun3mif3gJTQlLgSj7MmLhd4FFSVv6Cr6702eci4piE-uSQEG2ew6X3WqISQOWnSgqlUPv5GtZlSv0O9V28-LywmDNQJJT-bYmGu5UImOx3ZPHB7Q',
        'Content-Type': 'application/json'
    };

    const res = http.get(url, { headers });

    check(res, {
        'is status 200': (r) => r.status === 200,
        'response time < 500ms': (r) => r.timings.duration < 500
    });

}
