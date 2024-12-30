import React, { useState, useEffect, useRef } from 'react';
import { fetchGraphQL } from '../util/fetch-graphql';
import { GET_SUGGESTION, GET_BEST_COMBINATION } from '../util/queries';

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
                // TODO: Replace with fetchGraphQL
                const response = await fetch('http://localhost:8002/', {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify({
                        query: GET_SUGGESTION,
                        variables: { input: userInput },
                    }),
                });

                const json = await response.json();
                setSuggestion(json.data?.getSuggestion ?? undefined);
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

        await fetchBestCombination();
    };

    // Finalize input on "Enter" or selection
    const handleFinalizeInput = (input: string) => {
        if (!input.trim()) return;

        const finalValue = suggestion && suggestion.startsWith(input) ? suggestion : input;

        setSelectedItems(prev => [...prev, finalValue.trim()]);
        setUserInput('');
        setSuggestion('');
    };

    // Remove selected item
    const handleRemoveSelected = (item: string) => {
        setSelectedItems(prev => prev.filter(i => i !== item));
    };

    // Handle input field change
    const handleInputChange = (value: string) => {
        setUserInput(value);
    };

    return (
        <div className="autocomplete-container">
            <div style={{ position: 'relative', display: 'flex', alignItems: 'center' }}>
                {/* Input field */}
                <input
                    ref={inputRef}
                    className="input-field"
                    type="text"
                    placeholder="Type something..."
                    value={userInput}
                    onChange={e => handleInputChange(e.target.value)}
                    onKeyDown={e => {
                        if (e.key === 'Enter') {
                            e.preventDefault();
                            handleFinalizeInput(userInput);
                        }
                    }}
                    style={{ flex: 1 }} // Ensures the input takes available space
                />

                {/* Button */}
                <button onClick={handleButtonClick} style={{ marginLeft: '8px', padding: '8px 12px' }}>
                    Fetch
                </button>
            </div>

            {/* Ghost suggestion */}
            <div className="suggestion-ghost">{suggestion && suggestion.startsWith(userInput) ? suggestion : ''}</div>

            {/* Selected items */}
            <div className="selected-items">
                {selectedItems.map((item, index) => (
                    <div key={index} className="selected-item" onClick={() => handleRemoveSelected(item)}>
                        {item}
                    </div>
                ))}
            </div>

            <div className="best-combinations">
                {bestCombinations?.data?.map((combination, index) => (
                    <div key={index} className="combination">
                        <h4>Combination {index + 1}</h4>
                        <p>Coverage: {combination.combinedCoverage}%</p>
                        <p>Monthly Price: ${(combination.combinedMonthlyPriceCents / 100).toFixed(2)}</p>
                        <p>
                            Yearly Subscription Price: $
                            {(combination.combinedMonthlyPriceYearlySubscriptionInCents / 100).toFixed(2)}
                        </p>
                        <div className="packages">
                            {combination.packages.map((pkg, pkgIndex) => (
                                <div key={pkgIndex} className="package">
                                    <h5>{pkg.name}</h5>
                                    <p>Price: ${(pkg.monthlyPriceCents / 100).toFixed(2)}</p>
                                    <p>Yearly Price: ${(pkg.monthlyPriceYearlySubscriptionInCents / 100).toFixed(2)}</p>
                                    <div className="coverage">
                                        <h6>Coverage:</h6>
                                        {Object.entries(pkg.coverage).map(([competition, teams]) => (
                                            <p key={competition}>
                                                {competition}: {teams.join(', ')}
                                            </p>
                                        ))}
                                    </div>
                                </div>
                            ))}
                        </div>
                    </div>
                ))}
            </div>
        </div>
    );
};

export default AutocompleteSearch;
