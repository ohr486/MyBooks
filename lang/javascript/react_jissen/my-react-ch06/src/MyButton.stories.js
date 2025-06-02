import MyButton from './MyButton';
import { fn, userEvent, within } from 'storybook/test';
import { expect } from '@storybook/jest';

export default {
    title: 'MyApp/MyButton',
    component: MyButton,
    //tags: ['autodocs'],
    args: {
        onClick: fn()
    },
    argTypes: {
        primary: {
            type: 'boolean',
            description: 'Primaryカラーを有効にするか',
        },
        backgroundColor: {
            type: 'string',
            description: '背景色',
        },
        size: {
            type: {
                name: 'enum',
                value: ['small', 'medium', 'large']
            },
            control: { type: 'select' },
            description: 'ボタンの大きさ',
        },
        label: {
            type: 'string',
            description: 'ボタンのキャプション',
        },
        onClick: {
            type: 'function',
            description: 'clickハンドラー',
        },
        //handleClick: { action: 'clicked' }
    },
    parameters: {
        backgrounds: {
            options: {
                ghostwhite: { name: 'ghostwhite', value: '#f8f8ff' },
                aquamarine: { name: 'aquamarine', value: '#7fffd4' },
                coral: { name: 'coral', value: '#ff4f50' },
            },
        },
        layout: 'centered',
    },
    decorators: [
        Story => (
            <div style={{
                height: 150,
                display: 'flex',
                justifyContent: 'center',
                alignItems: 'center',
                backgroundColor: '#ccc'
            }}>
                <Story />
            </div>
        ),
    ],
};

//export const Index = {
//    render: () => <MyButton primary size="medium" label="ボタン"
//                            onClick={() => console.log('Hello, Storybook!!')} />
//};

export const Index = {
    //render: args => <MyButton {...args} />,
    args: {
        primary: true,
        size: 'medium',
        label: 'ボタン',
        //onClick: e => console.log(e, new Date())
    },
    play: async ({ args, canvasElement }) => {
        const canvas = within(canvasElement);
        const button = canvas.getByRole('button');
        await userEvent.click(button);
        await userEvent.click(button);
        await expect(args.onClick).toHaveBeenCalledTimes(2);
    }
};

//export const White = {
//    render: () => <MyButton size="small" label="ボタン"
//                            backgroundColor="#fff" />
//};

export const White = {
    args: {
        size: 'small',
        label: 'ボタン',
        backgroundColor: '#fff'
    }
};

export const Yellow = {
    args: {
        ...White.args,
        backgroundColor: 'lightyellow',
    }
};