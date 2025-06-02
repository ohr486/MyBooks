/** @type { import('@storybook/react-webpack5').Preview } */
import { INITIAL_VIEWPORTS} from "storybook/viewport";

const preview = {
  parameters: {
    actions: { argTypeRegex: "^(on|handle)[A-Z].*" },
    controls: {
      matchers: {
       color: /(background|color)$/i,
       date: /Date$/i,
      },
    },
    backgrounds: {
      options: {
        dark: { name: 'Dark', value: '#333' },
        light: { name: 'Light', value: '#F7F9F2' },
        ghostwhite: { name: 'ghostwhite', value: '#f8f8ff' },
        aquamarine: { name: 'aquamarine', value: '#7fffd4' },
        coral: { name: 'coral', value: '#ff4f50' },
      },
    },
    viewport: {
        viewports: INITIAL_VIEWPORTS,
        defaultViewport: 'iphonex',
    },
  },
};

export default preview;