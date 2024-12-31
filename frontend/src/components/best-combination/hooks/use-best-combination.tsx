import { useState, useCallback } from 'react';

import { fetchGraphQL } from '../../../util/fetch-graphql';
import { GET_BEST_COMBINATION } from '../../../util/queries';

import { BestCombinationStatus, DEFAULT_LIMIT } from '../types';
import type { BestCombinationResponse } from '../types';

interface UseBestCombinationReturn {
    bestCombinations: BestCombinationResponse | undefined;
    fetchBestCombination: (selectedItems: string[], limit?: number) => Promise<void>;
    error: string | undefined;
}

const useBestCombination = (): UseBestCombinationReturn => {
    const [bestCombinations, setBestCombinations] = useState<BestCombinationResponse | undefined>();
    const [error, setError] = useState<string | undefined>(undefined);

    const fetchBestCombination = useCallback(async (selectedItems: string[], limit: number = DEFAULT_LIMIT) => {
        const POLL_INTERVAL = 50; // in ms
        const TIMEOUT = 500; // in ms
        let elapsedTime = 0;

        const fetchWithPolling = async (): Promise<void> => {
            try {
                const response = await fetchGraphQL<{ getBestCombination: BestCombinationResponse }>(
                    GET_BEST_COMBINATION,
                    {
                        input: selectedItems,
                        opts: { limit },
                    },
                );

                const result = response.getBestCombination;

                if (result?.status === BestCombinationStatus.READY) {
                    // Sort the combinations by index in ascending order
                    const sortedData = result.data ? [...result.data].sort((a, b) => a.index - b.index) : undefined;

                    setBestCombinations({
                        status: result.status,
                        data: sortedData,
                    });
                    setError(undefined);
                    return;
                } else if (result?.status === BestCombinationStatus.PROCESSING && elapsedTime < TIMEOUT) {
                    elapsedTime += POLL_INTERVAL;
                    setTimeout(fetchWithPolling, POLL_INTERVAL);
                } else {
                    setError('Failed to fetch best combination: Timeout exceeded');
                }
            } catch (error) {
                setError(`Failed to reach URL. Please ensure that the backend is running`);
            }
        };

        if (selectedItems.length === 0) {
            setError('Please select at least one item.');
            return;
        }

        await fetchWithPolling();
    }, []);

    return { bestCombinations, fetchBestCombination, error };
};

export default useBestCombination;
