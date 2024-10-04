import { App } from "vue";

type Screen = {
  isSmall: boolean;
  isMedium: boolean;
  isMediumAndDown: boolean;
  isMediumAndUp: boolean;
  isLarge: boolean;
  isLargeAndDown: boolean;
  isLargeAndUp: boolean;
  isExtraLarge: boolean;
};


type Domain = {
  baseDomain: string
}

export const GlobalVariablesPlugin = {
  install: (app: App) => {
    const domain: Domain = {
      baseDomain: window.location.origin
    }

    const screen: Screen = {
      isSmall: window.innerWidth < 600,
      isMedium: window.innerWidth >= 600 && window.innerWidth < 1024,
      isMediumAndDown: window.innerWidth < 1024,
      isMediumAndUp: window.innerWidth >= 600,
      isLarge: window.innerWidth >= 1024 && window.innerWidth < 1440,
      isLargeAndDown: window.innerWidth < 1440,
      isLargeAndUp: window.innerWidth >= 1024,
      isExtraLarge: window.innerWidth >= 1440,
    };

    // Update screen values when the window is resized
    const updateScreenValues = () => {
      screen.isSmall = window.innerWidth < 600;
      screen.isMedium = window.innerWidth >= 600 && window.innerWidth < 1024;
      screen.isMediumAndDown = window.innerWidth < 1024;
      screen.isMediumAndUp = window.innerWidth >= 600;
      screen.isLarge = window.innerWidth >= 1024 && window.innerWidth < 1440;
      screen.isLargeAndDown = window.innerWidth < 1440;
      screen.isLargeAndUp = window.innerWidth >= 1024;
      screen.isExtraLarge = window.innerWidth >= 1440;
    };

    // Add screen object to globalProperties
    app.config.globalProperties.$domain = domain;
    app.config.globalProperties.$screen = screen;

    // Add event listener to update screen values on window resize
    window.addEventListener("resize", updateScreenValues);

    // Initialize screen values
    updateScreenValues();
  },
};
