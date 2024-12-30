export const ENQUEUE_BEST_COMBINATION = `
    mutation EnqueueBestCombination($input: [String!]!, $opts: FetchOptions!) {
      enqueueBestCombination(input: $input, opts: $opts)
    }
`;
