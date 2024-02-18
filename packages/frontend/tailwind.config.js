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
                    '50': '#fcf4ff',
                    '100': '#f8e6ff',
                    '200': '#f2d1ff',
                    '300': '#e8acff',
                    '400': '#d978ff',
                    '500': '#cb44ff',
                    '600': '#be20fd',
                    '700': '#a510de',
                    '800': '#8b13b6',
                    '900': '#711192',
                    '950': '#51006e',
                },
            },
            fontSize: {
                hero: '4.5rem',
                heroSubtitle: '1.5rem',
            },
            fontFamily: {
                'sans': ['"IBM Plex Sans"', "ui-sans-serif", "system-ui"]
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
