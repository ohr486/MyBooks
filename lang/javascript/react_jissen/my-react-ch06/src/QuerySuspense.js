import { useQuery } from '@tanstack/react-query';

const sleep = delay => new Promise(resolve => setTimeout(resolve, delay));

const APIKEY = process.env.REACT_APP_WEATHER_APIKEY;
const CITY = 'Tokyo';

const fetchWeather = async () => {
    await sleep(2000);
    const res = await fetch(`https://api.openweathermap.org/data/2.5/weather?q=${CITY}&lang=ja&appid=${APIKEY}`);
    if (res.ok) { return res.json(); }
    throw new Error(res.statusText);
};

export default function QueryBasic() {
    const { data } = useQuery({
        queryKey: ['weather'], queryFn: fetchWeather
    });
    return (
        <figure>
            <img
                src={`https://openweathermap.org/img/wn/${data?.weather?.[0]?.icon}.png`}
                alt={data?.weather?.[0]?.main} />
            <figcaption>{data?.weather?.[0]?.description}</figcaption>
        </figure>
    );
}