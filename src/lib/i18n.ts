import i18n from "i18next";
import { initReactI18next } from "react-i18next";
import id from "@/locales/id.json";
import en from "@/locales/en.json";
import { useSettings } from "@/stores/settings-store";

// Bahasa tersimpan sudah tersedia sinkron (localStorage), jadi tidak ada kedipan bahasa default.
const initial = useSettings.getState().language;
document.documentElement.lang = initial;

i18n.use(initReactI18next).init({
  resources: { id: { translation: id }, en: { translation: en } },
  lng: initial,
  fallbackLng: "id",
  interpolation: { escapeValue: false },
});

useSettings.subscribe(({ language }) => {
  if (language === i18n.language) return;
  i18n.changeLanguage(language);
  document.documentElement.lang = language;
});

export default i18n;
