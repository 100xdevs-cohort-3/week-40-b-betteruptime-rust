CREATE TYPE "WebsiteStatus" AS ENUM ('Up', 'Down', 'Unknown');

CREATE TABLE "website" (
    "id" TEXT NOT NULL,
    "url" TEXT NOT NULL,
    "name" TEXT,
    "time_added" TIMESTAMP(3) NOT NULL DEFAULT NOW (),
    CONSTRAINT "website_pkey" PRIMARY KEY ("id")
);

CREATE TABLE "region" (
    "id" TEXT NOT NULL,
    "name" TEXT NOT NULL,
    CONSTRAINT "region_pkey" PRIMARY KEY ("id")
);

CREATE TABLE "website_tick" (
    "id" TEXT NOT NULL,
    "response_time_ms" INTEGER NOT NULL,
    "status" "WebsiteStatus" NOT NULL,
    "region_id" TEXT NOT NULL,
    "website_id" TEXT NOT NULL,
    CONSTRAINT "website_tick_pkey" PRIMARY KEY ("id"),
    CONSTRAINT "website_tick_region_id_fkey" FOREIGN KEY ("region_id") REFERENCES "region" ("id") ON DELETE RESTRICT ON UPDATE CASCADE,
    CONSTRAINT "website_tick_website_id_fkey" FOREIGN KEY ("website_id") REFERENCES "website" ("id") ON DELETE RESTRICT ON UPDATE CASCADE
);
