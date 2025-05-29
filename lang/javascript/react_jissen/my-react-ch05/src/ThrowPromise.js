let flag = false;

export default function ThrowPromise() {
    if (flag) {
        return <p>正しく表示できました。</p>
    }

    throw new Promise((resolve, reject) => {
        setTimeout(() => {
            flag = true;
            resolve('Success!!');
            //reject(new Error('Error is occurred!!'));
        }, 3000);
    });
}