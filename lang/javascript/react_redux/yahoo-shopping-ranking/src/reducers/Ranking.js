const getRanking = response => {
    const ranking = [];
    const itemLength = response.totalResultsReturned;
    const items = response.hits;

    for (let index = 0; index < itemLength; index++) {
        const item = items[index];
        ranking.push({
            code: item.code,
            name: item.name,
            url: item.url,
            imageUrl: item.image.medium
        })
    }
    return ranking;
};

const initialState = {
    category: undefined,
    ranking: undefined,
    error: false
};

export default (state = initialState, action) => {
    switch (action.type) {
        case 'START_REQUEST':
            return {
                category: action.payload.category,
                ranking: undefined,
                error: false
            };

        case 'RECEIVE_DATA':
            return action.payload.error
                ? { ...state, error: true }
                : {
                    ...state,
                    ranking: getRanking(action.payload.response)
                };

        default:
            return state;
    }
}