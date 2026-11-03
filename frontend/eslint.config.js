import ts from 'typescript-eslint';
import vue from 'eslint-plugin-vue';
export default ts.config(
    {
        ignores: [
            'dist/**',
            'dist-ssr/**',
            'dist-server/**',
            'node_modules/**',
            'public/**',
        ],
    },
    ...ts.configs.recommended,
    ...vue.configs['flat/essential'],
    {
        files: ['**/*.vue'],
        languageOptions: {
            parserOptions: { parser: ts.parser, extraFileExtensions: ['.vue'] },
        },
    },
    {
        rules: {
            'vue/multi-word-component-names': 'off',
            '@typescript-eslint/no-explicit-any': 'error',
            '@typescript-eslint/ban-ts-comment': 'error',
        },
    },
    {
        files: ['src/domain/**/*.ts', 'src/application/**/*.ts'],
        ignores: ['**/*.spec.ts'],
        rules: {
            'no-restricted-imports': [
                'error',
                {
                    patterns: [
                        'vue',
                        'vue-router',
                        'vuetify',
                        '@/infrastructure/*',
                        '@/presentation/*',
                        '@/app/*',
                    ],
                },
            ],
        },
    },
);
