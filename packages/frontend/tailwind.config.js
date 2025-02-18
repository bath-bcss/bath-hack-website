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
					50: "#ecfdf7",
					100: "#d1faea",
					200: "#a6f4d9",
					300: "#6de8c6",
					400: "#33d4ac",
					500: "#0fba95",
					600: "#04977b",
					700: "#037965",
					800: "#056051",
					900: "#064e44",
					950: "#022c27",
				},
			},
			fontSize: {
				heroSubtitle: "1.5rem",
			},
			fontFamily: {
				sans: ['"IBM Plex Sans"', "ui-sans-serif", "system-ui"],
			},
			fontWeight: {
				hero: "800",
			},
		},
	},
	variants: {
		extend: {},
	},
	plugins: [],
};
