import React from 'react';

interface SelectedItemsProps {
    items: string[];
    onRemoveItem: (item: string) => void;
}

const SelectedItems: React.FC<SelectedItemsProps> = ({ items, onRemoveItem }) => {
    return (
        <div className="mt-4 flex flex-wrap gap-2">
            {items.map((item, index) => (
                <div
                    key={index}
                    className="
                        flex items-center
                        gap-2
                        rounded-full
                        border
                        bg-default
                        px-4 py-1
                        text-xs
                        tracking-tight
                        cursor-pointer
                        hover:line-through
                    "
                    onClick={() => onRemoveItem(item)}
                >
                    {item}
                </div>
            ))}
        </div>
    );
};

export default SelectedItems;
