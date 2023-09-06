const {
    override,
    addLessLoader,
    addWebpackAlias,
    adjustStyleLoaders,
} = require('customize-cra')
const path = require('path')

module.exports = override(
    addLessLoader({
        // lessOptions: {
        //     javascriptEnabled: true,
        // },
    }),
    adjustStyleLoaders(({use: [, , postcss]}) => {
        const postcssOptions = postcss.options;
        postcss.options = {postcssOptions};
    }),
    addWebpackAlias({
        '@': path.resolve(__dirname, 'src'),
    }),
)