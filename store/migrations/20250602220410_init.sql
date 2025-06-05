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
