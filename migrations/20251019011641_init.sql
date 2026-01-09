-- CreateTable
CREATE TABLE "api_keys" (
    "id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    "user_id" INTEGER NOT NULL,
    "name" TEXT NOT NULL,
    "digest" TEXT NOT NULL,
    "created_at" DATETIME NOT NULL
);

-- CreateTable
CREATE TABLE "attendance_records" (
    "id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    "workplace_id" INTEGER NOT NULL,
    "event" TEXT NOT NULL,
    "recorded_at" DATETIME NOT NULL,
    "created_at" DATETIME NOT NULL
);

-- CreateTable
CREATE TABLE "google_authenticated_users" (
    "id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    "user_id" INTEGER NOT NULL,
    "uid" TEXT NOT NULL,
    "created_at" DATETIME NOT NULL
);

-- CreateTable
CREATE TABLE "users" (
    "id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    "created_at" DATETIME NOT NULL
);

-- CreateTable
CREATE TABLE "workplaces" (
    "id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    "user_id" INTEGER NOT NULL,
    "name" TEXT NOT NULL,
    "created_at" DATETIME NOT NULL,
    "updated_at" DATETIME NOT NULL
);

-- CreateIndex
CREATE UNIQUE INDEX "index_api_keys_on_digest" ON "api_keys"("digest");

-- CreateIndex
CREATE INDEX "index_api_keys_on_user_id" ON "api_keys"("user_id");

-- CreateIndex
CREATE INDEX "index_attendance_records_on_workplace_id" ON "attendance_records"("workplace_id");

-- CreateIndex
CREATE UNIQUE INDEX "index_google_authenticated_users_on_uid" ON "google_authenticated_users"("uid");

-- CreateIndex
CREATE INDEX "index_google_authenticated_users_on_user_id" ON "google_authenticated_users"("user_id");

-- CreateIndex
CREATE INDEX "index_workplaces_on_user_id" ON "workplaces"("user_id");
