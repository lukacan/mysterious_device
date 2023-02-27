COUNT=$1
MAX=$2
FILE="input_${COUNT}"
RANODM=$$
echo $COUNT > $FILE
for i in `seq $COUNT`
do
	shuf -i 4-$MAX -n 1 >> $FILE
done


