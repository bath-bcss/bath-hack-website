module.exports = {
    mode: "jit",
    content: {
        files: ["src/**/*.rs", "index.html"],
    },
    darkMode: "media", // 'media' or 'class'
    theme: {
        extend: {
            colors: {
                bcss: {
                    '50': '#f1f1ff',
                    '100': '#e6e6ff',
                    '200': '#d0d0ff',
                    '300': '#aeabff',
                    '400': '#867cff',
                    '500': '#6047ff',
                    '600': '#4b21ff',
                    '700': '#3d10f1',
                    '800': '#340dd3',
                    '900': '#2a0ca6',
                    '950': '#160471',
                },
            },
            fontSize: {
                hero: '4rem',
            },
            fontFamily: {
                'sans': ['"Sometype Mono"', "ui-sans-serif", "system-ui"]
            },
            fontWeight: {
                hero: '800',
            }
        },
    },
    variants: {
        extend: {},
    },
    plugins: [],
};
