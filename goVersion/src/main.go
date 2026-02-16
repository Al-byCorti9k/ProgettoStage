package main

import (
	"fmt"
	"log"

	//"os"

	dataprocess "github.com/Al-byCorti9k/ProgettoStage/goVersion/src/data_process"

	cli "github.com/Al-byCorti9k/ProgettoStage/goVersion/src/cli_tools"
)

func main() {

	args := cli.ParseCliArgument()
	log.SetPrefix("go_version: ")
	log.SetFlags(0)
	fmt.Print(args)
	//a, err := dataprocess.GetDatasetInfo(nil)

	num := args.DArgs[0]
	fmt.Println(num)
	df := dataprocess.GetDataframeFromID(args.DArgs[0])

	dfInfo := dataprocess.DataframeInfoBuild(args.DArgs[0], &df)

	var targetColumn string

	if len(args.CArgs) != 0 {
		cli.InputColumnsCheck(&dfInfo, &args.CArgs[0])
		targetColumn = args.CArgs[0]
	} else {
		_, numberCols := dfInfo.Df.Dims()
		targetColumn = dfInfo.Df.Names()[numberCols-1]
	}
	println(targetColumn)

	fmt.Println(dfInfo)

	dfInfo2 := dataprocess.FillColumnsNanValues(&dfInfo)

	fmt.Println(dfInfo2)

	//fmt.Println(a.VecToHashSet())
	fmt.Println(dfInfo.Df.Names())

	/*
		f, err := os.Create("output.csv")
		if err != nil {
			log.Fatal(err)
		}
		defer f.Close()

		// 3. Esportiamo il DataFrame nel file
		err = dfInfo2.Df.WriteCSV(f)
		if err != nil {
			log.Fatal(err)
		}
	*/
}
