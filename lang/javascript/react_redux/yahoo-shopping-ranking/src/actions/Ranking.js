import fetchJsonp from 'fetch-jsonp';
import qs from 'qs';
import { replace } from 'react-router-redux';
const { APP_ID, API_URL } = process.env;

const startRequest = category => ({
    type: 'START_REQUEST',
    payload: { category },
});

const receiveData = (category, error, response) => ({
    type: 'RECEIVE_DATA',
    payload: { category, error, response },
});

const finishRequest = category => ({
    type: 'FINISH_REQUEST',
    payload: { category },
});

export const fetchRanking = categoryId => {
    return async (dispatch, getState) => {
        const categories = getState().shopping.categories;

        const category = categories.find(category => (category.id === categoryId));

        if (typeof category === 'undefined') {
            dispatch(replace('/'));
            return;
        }

        dispatch(startRequest(category));

        const queryString = qs.stringify({
            appid: APP_ID,
            query: 'nike',
            genre_category_id: categoryId,
        });

        try {
            const response = await fetch(`${API_URL}?${queryString}`);
            if (!response.ok) {
                throw new Error(`HTTP error! status: ${response.status}`);
            }
            const data = await response.json();
            dispatch(receiveData(category, null, data));
        } catch (err) {
            dispatch(receiveData(category, err));
        }

        dispatch(finishRequest(category));
    };
};
