import React from 'react';

import { CombinationCard } from './best-combination-card';
import type { BestCombination } from './types';

interface BestCombinationListProps {
    combinations: BestCombination[];
}

const BestCombinationList: React.FC<BestCombinationListProps> = ({ combinations }) => {
    return (
        <div className="mt-8">
            <h2 className="text-xl font-bold mb-4">Results</h2>
            {combinations.map((combination, idx) => (
                <CombinationCard key={idx} combination={combination} index={idx} />
            ))}
        </div>
    );
};

export default BestCombinationList;
