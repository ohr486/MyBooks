import React from 'react';
import renderer from 'react-test-renderer';
import HelloWorld from './HelloWorld';

test('Hello Worldコンポーネントのスナップショットテスト', () => {
    const result = renderer.create(<HelloWorld />).toJSON();
    expect(result).toMatchSnapshot();
});