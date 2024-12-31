import React, { useRef, useEffect, useState } from 'react';

import useSuggestion from './hooks/use-suggestion';
import useBestCombination from './hooks/use-best-combination';
import useFinalizeInput from './hooks/use-finalize-input';

import SearchInput from './search-input';
import SelectedItems from './selected-items';
import BestCombinationList from './best-combination-list';

const BestCombinationContainer: React.FC = () => {
    const inputRef = useRef<HTMLInputElement | null>(null);

    // Custom Hooks
    const { userInput, setUserInput, suggestion } = useSuggestion();
    const { selectedItems, addItem, removeItem } = useFinalizeInput();
    const { bestCombinations, fetchBestCombination, error } = useBestCombination();

    // Local state for displaying the error
    const [visibleError, setVisibleError] = useState<string | undefined>(undefined);

    // Effect to handle error visibility and auto-dismiss
    useEffect(() => {
        if (error) {
            setVisibleError(error);

            // Set a timer to clear the error after 10 seconds
            const timer = setTimeout(() => {
                setVisibleError(undefined);
            }, 10000);

            // Clear the timer if the component unmounts or error changes
            return () => clearTimeout(timer);
        }
    }, [error]);

    // Handle Button Click
    const handleButtonClick = async () => {
        await fetchBestCombination(selectedItems);
    };

    // Finalize Input
    const handleFinalizeInput = async (input: string) => {
        setUserInput('');
        await addItem(input, suggestion);
    };

    // Remove Selected Item
    const handleRemoveSelected = async (item: string) => {
        await removeItem(item);
    };

    // Handle Input Change
    const handleInputChange = (value: string) => {
        setUserInput(value);
    };

    return (
        <div className="w-full max-w-2xl mx-auto p-4">
            {/* Row that holds both the search box and the button */}
            <div className="flex items-center gap-2">
                <SearchInput
                    userInput={userInput}
                    suggestion={suggestion}
                    onInputChange={handleInputChange}
                    onFinalizeInput={handleFinalizeInput}
                    inputRef={inputRef}
                />
                <button
                    onClick={handleButtonClick}
                    className="
                        h-10
                        rounded-md
                        font-bold
                        border-2
                        border-border
                        px-4
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

            {/* Error Message */}
            {visibleError && <div className="mt-2 text-red-600 text-xs">{visibleError}</div>}

            {/* Selected items */}
            <SelectedItems items={selectedItems} onRemoveItem={handleRemoveSelected} />

            {/* Best Combination UI */}
            {bestCombinations?.data && <BestCombinationList combinations={bestCombinations.data} />}
        </div>
    );
};

export default BestCombinationContainer;
