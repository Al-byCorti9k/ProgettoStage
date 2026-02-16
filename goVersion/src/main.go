package main

import (
	"fmt"
	"log"

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
	println(args.CArgs[0])
	cli.InputColumnsCheck(&dfInfo, &args.CArgs[0])

	fmt.Println(dfInfo)

	//fmt.Println(a.VecToHashSet())
	fmt.Println(dfInfo.Df.Names())

}
