import React from 'react';

interface SearchInputProps {
    userInput: string;
    suggestion?: string;
    onInputChange: (value: string) => void;
    onFinalizeInput: (input: string) => void;
    inputRef: React.RefObject<HTMLInputElement | null>;
}

const SearchInput: React.FC<SearchInputProps> = ({
    userInput,
    suggestion,
    onInputChange,
    onFinalizeInput,
    inputRef,
}) => {
    return (
        <div
            className="relative w-full h-10 gap-1 rounded-md border-2 border-border bg-background px-3 text-text focus-within:border-primary transition-colors"
            style={{ fontSize: '1rem' }}
        >
            {/* Suggestion overlay */}
            {suggestion && suggestion.startsWith(userInput) && (
                <div className="absolute inset-0 flex items-center pointer-events-none px-3">
                    <span className="text-gray-400">
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
                onChange={e => onInputChange(e.target.value)}
                onKeyDown={e => {
                    if (e.key === 'Enter') {
                        e.preventDefault();
                        onFinalizeInput(userInput);
                    }
                }}
            />
        </div>
    );
};

export default SearchInput;
