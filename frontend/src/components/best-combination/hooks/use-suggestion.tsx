import { useState, useEffect, useRef } from 'react';

import { fetchGraphQL } from '../../../util/fetch-graphql';
import { GET_SUGGESTION } from '../../../util/queries';

const debounceInterval = 100;

const useSuggestion = () => {
    const [userInput, setUserInput] = useState('');
    const [suggestion, setSuggestion] = useState<string | undefined>();
    const debounceTimer = useRef<number | null>(null);

    useEffect(() => {
        if (userInput.trim() === '') {
            setSuggestion(undefined);
            return;
        }

        const fetchSuggestion = async () => {
            try {
                const data = await fetchGraphQL<{ getSuggestion: string }>(GET_SUGGESTION, { input: userInput });
                setSuggestion(data.getSuggestion ?? undefined);
            } catch (error) {
                alert(`Failed to fetch suggestion: ${error}`);
                setSuggestion(undefined);
            }
        };

        // Debounce the fetch call
        if (debounceTimer.current) {
            clearTimeout(debounceTimer.current);
        }
        debounceTimer.current = window.setTimeout(fetchSuggestion, debounceInterval);

        return () => {
            if (debounceTimer.current) clearTimeout(debounceTimer.current);
        };
    }, [userInput]);

    return { userInput, setUserInput, suggestion };
};

export default useSuggestion;
