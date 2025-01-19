# Beende das Skript sicher mit CTRL+C
trap "echo 'Beendet mit CTRL+C'; exit" INT

# Laufzeit setzen: 2 Stunden + 10 Minuten (7800 Sekunden)
end=$((SECONDS + 7800))

while [ $SECONDS -lt $end ]; do
  docker stats --no-stream --format "{{.CPUPerc}},{{.MemUsage}}" pflanzenshop-checkoutService-1 | \
  sed 's/ \/.*//' >> dockerstats_java.csv
done

echo "Skript nach 2 Stunden und 10 Minuten beendet."

