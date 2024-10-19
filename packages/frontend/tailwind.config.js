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
					50: "#f1f6fd",
					100: "#dfebfa",
					200: "#c6dbf7",
					300: "#99c1f1",
					400: "#70a5ea",
					500: "#4f84e2",
					600: "#3a68d6",
					700: "#3154c4",
					800: "#2d46a0",
					900: "#293f7f",
					950: "#1d284e",
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
