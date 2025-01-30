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
					50: "#fdf6ef",
					100: "#fbead9",
					200: "#f6d0af",
					300: "#f0b281",
					400: "#e9894e",
					500: "#e46a2b",
					600: "#d65120",
					700: "#b13e1d",
					800: "#8e321e",
					900: "#722c1c",
					950: "#3e130c",
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
