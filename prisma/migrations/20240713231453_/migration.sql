-- CreateTable
CREATE TABLE "CurrencyPrice" (
    "name" TEXT NOT NULL,
    "date" TEXT NOT NULL,
    "price" DOUBLE PRECISION NOT NULL,

    CONSTRAINT "CurrencyPrice_pkey" PRIMARY KEY ("name")
);

-- CreateIndex
CREATE UNIQUE INDEX "CurrencyPrice_name_date_key" ON "CurrencyPrice"("name", "date");
