/// <reference path="../.astro/types.d.ts" />
interface ImportMetaEnv {
    readonly APOLLO_URL: string;
}

interface ImportMeta {
    readonly env: ImportMetaEnv;
}
