import { useState } from 'react';

import { fetchGraphQL } from '../../../util/fetch-graphql';
import { ENQUEUE_BEST_COMBINATION } from '../../../util/mutations';
import { DEFAULT_LIMIT } from '../types';

interface UseFinalizeInputReturn {
    selectedItems: string[];
    addItem: (input: string, suggestion?: string) => Promise<void>;
    removeItem: (item: string) => Promise<void>;
}

const useFinalizeInput = (initialSelectedItems: string[] = []): UseFinalizeInputReturn => {
    const [selectedItems, setSelectedItems] = useState<string[]>(initialSelectedItems);

    const addItem = async (input: string, suggestion?: string) => {
        if (!input.trim()) return;

        const finalValue = suggestion && suggestion.startsWith(input) ? suggestion : input.trim();

        // Prevent duplicates
        if (selectedItems.includes(finalValue)) {
            return;
        }

        const updatedSelectedItems = [...selectedItems, finalValue];
        setSelectedItems(updatedSelectedItems);

        // Enqueue current selection via mutation
        try {
            const result = await fetchGraphQL<{ enqueueBestCombination: string }>(ENQUEUE_BEST_COMBINATION, {
                input: updatedSelectedItems,
                opts: { limit: DEFAULT_LIMIT },
            });

            if (result.enqueueBestCombination === 'ERROR') {
                console.error('Failed to enqueue the best combination.');
            }
        } catch (error) {
            console.error(`Mutation error: ${error}`);
        }
    };

    const removeItem = async (item: string) => {
        const updatedSelectedItems = selectedItems.filter(i => i !== item);
        setSelectedItems(updatedSelectedItems);

        // Send the mutation to enqueue the updated selection
        try {
            const result = await fetchGraphQL<{ enqueueBestCombination: string }>(ENQUEUE_BEST_COMBINATION, {
                input: updatedSelectedItems,
                opts: { limit: 1 },
            });

            if (result.enqueueBestCombination === 'ERROR') {
                console.error('Failed to enqueue the updated selection after removing an item.');
            }
        } catch (error) {
            console.error(`Mutation error after removing an item: ${error}`);
        }
    };

    return { selectedItems, addItem, removeItem };
};

export default useFinalizeInput;
