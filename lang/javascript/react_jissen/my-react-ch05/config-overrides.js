const { addBabelPlugins, override } = require('customize-cra');
module.exports = override(
    ...addBabelPlugins(
        "styled-jsx/babel"
    )
);