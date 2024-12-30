interface GraphQLError {
    message: string;
}

export async function fetchGraphQL<TData>(query: string, variables?: Record<string, unknown>): Promise<TData> {
    // TODO: Use env url for container
    const response = await fetch('http://localhost:4000', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify({ query, variables }),
    });

    const result = (await response.json()) as { data: TData; errors?: GraphQLError[] };

    if (result.errors && result.errors.length > 0) {
        console.error('GraphQL Errors:', result.errors);
        throw new Error(result.errors.map(err => err.message).join('\n'));
    }

    return result.data;
}
