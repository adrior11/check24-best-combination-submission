import React, { useState, useEffect, useRef } from 'react';
import { fetchGraphQL } from '../scripts/fetch-graphql';
import { GET_SUGGESTION } from '../scripts/queries';

const AutocompleteSearch: React.FC = () => {
    const [userInput, setUserInput] = useState('');
    const [suggestion, setSuggestion] = useState<string | undefined>();
    const [selectedItems, setSelectedItems] = useState<string[]>([]);
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
                const response = await fetch("http://localhost:8002/", 
                    {
                    method: 'POST',
                    headers: {'Content-Type':'application/json'},
                    body: JSON.stringify({
                        query:  GET_SUGGESTION,
                        variables: { input: userInput },
                    })
                })

                const json = await response.json();
                setSuggestion(json.data?.getSuggestion ?? undefined);
            } catch (error) {
                alert(`Failed to fetch from service: ${error}`);
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

    // Finalize input on "Enter" or selection
    const handleFinalizeInput = (input: string) => {
        if (!input.trim()) return;

        const finalValue = suggestion && suggestion.startsWith(input) 
            ? suggestion 
            : input; 

        setSelectedItems((prev) => [...prev, finalValue.trim()]);
        setUserInput('');
        setSuggestion('');
    };

    // Remove selected item
    const handleRemoveSelected = (item: string) => {
        setSelectedItems((prev) => prev.filter((i) => i !== item));
    };

    // Handle input field change
    const handleInputChange = (value: string) => {
        setUserInput(value);
    };

    return (
        <div className="autocomplete-container">
            <div style={{ position: 'relative' }}>
                {/* Input field */}
                <input
                    ref={inputRef}
                    className="input-field"
                    type="text"
                    placeholder="Type something..."
                    value={userInput}
                    onChange={(e) => handleInputChange(e.target.value)}
                    onKeyDown={(e) => {
                        if (e.key === 'Enter') {
                            e.preventDefault();
                            handleFinalizeInput(userInput);
                        }
                    }}
                />

                {/* Ghost suggestion */}
                <div className="suggestion-ghost">
                    {suggestion && suggestion.startsWith(userInput) ? suggestion : ''}
                </div>
            </div>

            {/* Selected items */}
            <div className="selected-items">
                {selectedItems.map((item, index) => (
                    <div
                        key={index}
                        className="selected-item"
                        onClick={() => handleRemoveSelected(item)}
                    >
                        {item}
                    </div>
                ))}
            </div>
        </div>
    );
};

export default AutocompleteSearch;
