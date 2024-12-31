export const GET_TEAMS = `
    query GetTeams {
        getTeams
    }
`;

export const GET_TOURNAMENTS = `
    query GetTournaments {
        getTournaments
    }
`;

export const GET_SUGGESTION = `
    query GetSuggestion($input: String!) {
        getSuggestion(input: $input)
    }
`;

export const GET_BEST_COMBINATION = `
    query GetBestCombination($input: [String!]!, $opts: FetchOptions!) {
        getBestCombination(input: $input, opts: $opts) {
            status
            data {
                index
                combinedCoverage
                combinedMonthlyPriceCents
                combinedMonthlyPriceYearlySubscriptionInCents
                packages {
                    name
                    coverage
                    monthlyPriceCents
                    monthlyPriceYearlySubscriptionInCents
                }
            }
        }
    }
`;
