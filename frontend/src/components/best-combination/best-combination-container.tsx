import React, { useRef } from 'react';

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
    const { bestCombinations, fetchBestCombination } = useBestCombination();

    // Handle Button Click
    const handleButtonClick = async () => {
        await fetchBestCombination(selectedItems);
    };

    // Finalize Input
    const handleFinalizeInput = async (input: string) => {
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
            <SelectedItems items={selectedItems} onRemoveItem={handleRemoveSelected} />

            {/* Best Combination UI */}
            {bestCombinations?.data && <BestCombinationList combinations={bestCombinations.data} />}
        </div>
    );
};

export default BestCombinationContainer;
