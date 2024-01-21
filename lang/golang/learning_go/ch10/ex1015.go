package main

import (
	"context"
	"fmt"
	"log"
	"math/rand"
	"os"
	"time"
)

type AOut int
type BOut int
type COut int
type CIn struct {
	A AOut
	B BOut
}
type Input int

type processor struct {
	outA chan AOut
	outB chan BOut
	outC chan COut
	inC  chan CIn
	errs chan error
}

const maxInt = 20
const timeLimit = 50

func gatherAndProcess(ctx context.Context, data Input) (COut, error) {
	log.SetFlags(log.Lmicroseconds)
	log.Println("starting; timeLimit:", timeLimit*time.Millisecond)
	ctx, cancel := context.WithTimeout(ctx, timeLimit*time.Millisecond)
	defer cancel()

	p := processor{
		outA: make(chan AOut, 1),
		outB: make(chan BOut, 1),
		inC:  make(chan CIn, 1),
		outC: make(chan COut, 1),
		errs: make(chan error, 2),
	}
	p.launch(ctx, data)
	inputC, err := p.waitForAB(ctx)
	if err != nil {
		fmt.Println("Error returned from waitForAB:", err)
		os.Exit(1)
	}
	p.inC <- inputC
	out, err := p.waitForC(ctx)
	return out, err
}

func (p *processor) launch(ctx context.Context, data Input) {
	go func() {
		aOut, err := getResultA(ctx, data)
		if err != nil {
			p.errs <- err
			return
		}
		p.outA <- aOut
	}()

	go func() {
		bOut, err := getResultB(ctx, data)
		if err != nil {
			p.errs <- err
			return
		}
		p.outB <- bOut
	}()

	go func() {
		select {
		case <-ctx.Done():
			log.Println("case ctx.Done()")
			return
		case inputC := <-p.inC:
			cOut, err := getResultC(ctx, inputC)
			if err != nil {
				p.errs <- err
				return
			}
			p.outC <- cOut
		}
	}()
}

func (p *processor) waitForAB(ctx context.Context) (CIn, error) {
	var inputC CIn
	count := 0
	for count < 2 {
		select {
		case a := <-p.outA:
			inputC.A = a
			count++
		case b := <-p.outB:
			inputC.B = b
			count++
		case err := <-p.errs:
			return CIn{}, err
		case <-ctx.Done():
			return CIn{}, ctx.Err()
		}
	}
	return inputC, nil
}

func (p *processor) waitForC(ctx context.Context) (COut, error) {
	select {
	case out := <-p.outC:
		return out, nil
	case err := <-p.errs:
		return 0, err
	case <-ctx.Done():
		return 0, ctx.Err()
	}
}

func main() {
	rand.Seed(time.Now().UnixNano())
	ctx := context.Background()

	n := Input(rand.Intn(maxInt) + 1)
	v, err := gatherAndProcess(ctx, n)
	if err != nil {
		fmt.Println("Error: ", err)
		return
	}
	log.Println("Final Result:", v)
}

func getResultA(ctx context.Context, data Input) (AOut, error) {
	time.Sleep(randomPeriod("A", 1))
	r := AOut(data * Input(rand.Intn(maxInt)+1))
	log.Println("from A:", r)
	return r, nil
}

func getResultB(ctx context.Context, data Input) (BOut, error) {
	time.Sleep(randomPeriod("B", 1))
	r := BOut(data * Input(rand.Intn(maxInt)+1))
	log.Println("from B:", r)
	return r, nil
}

func getResultC(ctx context.Context, data CIn) (COut, error) {
	time.Sleep(randomPeriod("C", 3))
	r := int(data.A) + int(data.B)
	return COut(r), nil
}

func randomPeriod(which string, div int) time.Duration {
	i := rand.Intn(timeLimit)/div + 1 + timeLimit/10
	t := time.Millisecond * time.Duration(i)
	log.Println(which, "will sleep:", t)
	return t
}
