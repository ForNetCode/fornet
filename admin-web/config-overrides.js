const {
    override,
    fixBabelImports,
    addLessLoader,
    addWebpackAlias,
    adjustStyleLoaders,
} = require('customize-cra')
const {getThemeVariables} = require('antd/dist/theme');
const path = require('path')

module.exports = override(
    fixBabelImports('import', {
        libraryName: 'antd',
        libraryDirectory: 'es',
        style: true,
    }),
    addLessLoader({
        lessOptions: {
            javascriptEnabled: true,
            modifyVars: {
                ...getThemeVariables({
                    dark: true,
                }),
            },
        },
    }),
    adjustStyleLoaders(({use: [, , postcss]}) => {
        const postcssOptions = postcss.options;
        postcss.options = {postcssOptions};
    }),
    addWebpackAlias({
        '@': path.resolve(__dirname, 'src'),
    }),
)
