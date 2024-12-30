import React, { useState, useEffect, useRef } from 'react';

import { fetchGraphQL } from '../util/fetch-graphql';
import { GET_SUGGESTION, GET_BEST_COMBINATION } from '../util/queries';
import { ENQUEUE_BEST_COMBINATION } from '../util/mutations';

import { CombinationCard } from './combination-card';

interface Coverage {
    [key: string]: number[];
}

interface Package {
    name: string;
    coverage: Coverage;
    monthlyPriceCents: number;
    monthlyPriceYearlySubscriptionInCents: number;
}

interface BestCombination {
    combinedCoverage: number;
    combinedMonthlyPriceCents: number;
    combinedMonthlyPriceYearlySubscriptionInCents: number;
    packages: Package[];
}

enum BestCombinationStatus {
    'READY',
    'PROCESSING',
    'ERROR',
}

interface BestCombinationResponse {
    status: BestCombinationStatus;
    data: BestCombination[] | undefined;
}

const AutocompleteSearch: React.FC = () => {
    const [userInput, setUserInput] = useState('');
    const [suggestion, setSuggestion] = useState<string | undefined>();
    const [selectedItems, setSelectedItems] = useState<string[]>([]);
    const [bestCombinations, setBestCombinations] = useState<BestCombinationResponse | undefined>();
    const inputRef = useRef<HTMLInputElement | null>(null);

    const debounceTimer = useRef<number | null>(null);

    // Fetch suggestion based on user input
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
        debounceTimer.current = window.setTimeout(fetchSuggestion, 100);

        return () => {
            if (debounceTimer.current) clearTimeout(debounceTimer.current);
        };
    }, [userInput]);

    // Submit best combination
    const handleButtonClick = async () => {
        const POLL_INTERVAL = 50; // Poll every 50ms
        const TIMEOUT = 500; // Stop polling after 500ms
        let elapsedTime = 0;

        const fetchBestCombination = async (): Promise<void> => {
            try {
                // TODO: Replace with fetchGraphQL
                const response = await fetch('http://localhost:8001/', {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify({
                        query: GET_BEST_COMBINATION,
                        variables: {
                            input: selectedItems,
                            opts: {
                                limit: 1,
                            },
                        },
                    }),
                });

                const json = await response.json();
                const result = json.data?.getBestCombination;

                if (result?.status === 'READY') {
                    setBestCombinations(result);
                    return;
                } else if (result?.status === 'PROCESSING' && elapsedTime < TIMEOUT) {
                    elapsedTime += POLL_INTERVAL;
                    setTimeout(fetchBestCombination, POLL_INTERVAL);
                } else {
                    alert('Failed to fetch best combination: Timeout exceeded');
                }
            } catch (error) {
                alert(`Failed to fetch best combination: ${error}`);
            }
        };

        if (selectedItems.length === 0) {
            alert('Please select at least one item.');
            return;
        }

        await fetchBestCombination();
    };

    // Finalize input on "Enter" or selection
    const handleFinalizeInput = async (input: string) => {
        if (!input.trim()) return;

        const finalValue = suggestion && suggestion.startsWith(input) ? suggestion : input;

        // Prevent duplicates
        if (selectedItems.includes(finalValue.trim())) {
            setUserInput('');
            setSuggestion('');
            return;
        }

        const updatedSelectedItems = [...selectedItems, finalValue.trim()];
        setSelectedItems(updatedSelectedItems);
        setUserInput('');
        setSuggestion('');

        // Enqueue current selection via mutation
        try {
            const response = await fetch('http://localhost:8001/', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({
                    query: ENQUEUE_BEST_COMBINATION,
                    variables: {
                        input: updatedSelectedItems,
                        opts: {
                            limit: 1,
                        },
                    },
                }),
            });

            const json = await response.json();
            const result = json.data?.enqueueBestCombination;

            if (result === 'ERROR') {
                console.error('Failed to enqueue the best combination.');
            }
        } catch (error) {
            console.error(`Mutation error: ${error}`);
        }
    };

    // Remove selected item
    const handleRemoveSelected = async (item: string) => {
        // Update the selected items locally
        const updatedSelectedItems = selectedItems.filter(i => i !== item);
        setSelectedItems(updatedSelectedItems);

        // Send the mutation to enqueue the updated selection
        try {
            const response = await fetch('http://localhost:8001/', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({
                    query: `
                    mutation EnqueueBestCombination($input: [String!]!, $opts: FetchOptions!) {
                      enqueueBestCombination(input: $input, opts: $opts)
                    }
                `,
                    variables: {
                        input: updatedSelectedItems,
                        opts: {
                            limit: 1,
                        },
                    },
                }),
            });

            const json = await response.json();
            const result = json.data?.enqueueBestCombination;

            if (result === 'ERROR') {
                console.error('Failed to enqueue the updated selection after removing an item.');
            }
        } catch (error) {
            console.error(`Mutation error after removing an item: ${error}`);
        }
    };

    // Handle input field change
    const handleInputChange = (value: string) => {
        setUserInput(value);
    };

    return (
        <div className="w-full max-w-2xl mx-auto p-4">
            {/* Row that holds both the search box and the button */}
            <div className="flex items-center gap-2">
                {/* Search box container, grows to fill space */}
                {/* className="items-center flex-1 h-10 gap-1 rounded-md border-2 border-border bg-background px-3 
                text-text focus-within:border-primary transition-colors" */}
                <div
                    className="relative w-full h-10 gap-1 rounded-md border-2 border-border bg-background px-3 text-text focus-within:border-primary transition-colors"
                    style={{ fontSize: '1rem' }}
                >
                    {/* Suggestion overlay */}
                    {suggestion && suggestion.startsWith(userInput) && (
                        <div className="absolute inset-0 flex items-center pointer-events-none px-3">
                            <span className="text-gray-400">
                                {/* Render the full suggestion */}
                                <span>{userInput}</span>
                                <span>{suggestion.substring(userInput.length)}</span>
                            </span>
                        </div>
                    )}

                    {/* Input field */}
                    <input
                        ref={inputRef}
                        type="text"
                        className="w-full bg-transparent outline-none text-left h-full z-10 relative caret-black"
                        placeholder="Type something..."
                        value={userInput}
                        onChange={e => handleInputChange(e.target.value)}
                        onKeyDown={e => {
                            if (e.key === 'Enter') {
                                e.preventDefault();
                                handleFinalizeInput(userInput);
                            }
                        }}
                    />
                </div>
                {/* Submission button, same height as the search box */}
                <button
                    onClick={handleButtonClick}
                    className="
          h-10
          rounded-md
          bg-primary
          px-4
          text-white
          hover:opacity-90
          transition-opacity
          focus:outline-none
          focus:ring-2
          focus:ring-primary
          focus:ring-offset-2
        "
                >
                    Search
                </button>
            </div>

            {/* Selected items */}
            <div className="mt-4 flex flex-wrap gap-2">
                {selectedItems.map((item, index) => (
                    <div
                        key={index}
                        className="
            flex items-center
            gap-2
            rounded-full
            border border-current
            bg-default
            px-4 py-1
            text-xs
            tracking-tight
            cursor-pointer
          "
                        onClick={() => handleRemoveSelected(item)}
                    >
                        {item}
                        <span className="text-text-offset hover:text-text">✕</span>
                    </div>
                ))}
            </div>

            {/* (Ignore Best Combination UI for now) */}
            {bestCombinations?.data && (
                <div className="mt-8">
                    <h2 className="text-xl font-bold mb-4">Best Combinations</h2>
                    {bestCombinations.data.map((combination, idx) => (
                        <CombinationCard key={idx} combination={combination} index={idx} />
                    ))}
                </div>
            )}
        </div>
    );
};

export default AutocompleteSearch;
