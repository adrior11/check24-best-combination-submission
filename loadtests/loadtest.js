import http from "k6/http";
import { check, sleep } from "k6";
import { SharedArray } from "k6/data";

const data = new SharedArray("loadTestData", () =>
  JSON.parse(open("./data.json")).map((team) => team.team_name),
);

export const options = {
  stages: [
    { duration: "30s", target: 50 },
    { duration: "2m", target: 100 },
    { duration: "30s", target: 0 },
  ],
  thresholds: {
    http_req_duration: ["p(99)<500"],
    http_req_failed: ["rate<0.01"],
  },
};

export default function () {
  // Randomly select a number of teams between 10 and 100
  const numTeams = Math.floor(Math.random() * 91) + 10;
  const selectedTeams = [];

  for (let i = 0; i < numTeams; i++) {
    const teamIndex = Math.floor(Math.random() * data.length);
    selectedTeams.push(data[teamIndex]);
  }

  const query = `
    query GetBestCombination($teams: [String!]!, $opts: FetchOptions) {
      getBestCombination(teams: $teams, opts: $opts) {
        status
        data {
          packages
          combinedMonthlyPriceCents
          combinedMonthlyPriceYearlySubscriptionInCents
          coverage
        }
      }
    }
  `;

  const limit = Math.floor(Math.random() * 3) + 1;

  const variables = {
    teams: selectedTeams,
    opts: {
      limit: limit,
    },
  };

  const payload = JSON.stringify({
    query: query,
    variables: variables,
  });

  const params = {
    headers: {
      "Content-Type": "application/json",
    },
  };

  const res = http.post("http://localhost:8000/graphql", payload, params);

  check(res, {
    "status is 200": (r) => r.status === 200,
    "response time is < 500ms": (r) => r.timings.duration < 500,
    "response has data": (r) => {
      try {
        const json = JSON.parse(r.body);
        return json.data !== null && json.data.best_combination !== null;
      } catch (e) {
        return false;
      }
    },
  });

  sleep(Math.random() * 2);
}
