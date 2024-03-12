import os, sys
import math, time
import h5py
from array import array
import numpy as np
import ROOT
from ROOT import TFile, TTree, TLorentzVector
import glob
# import subprocess
# from subprocess import call
import argparse

meMeV = 0.5109989461 ## MeV
meGeV = meMeV/1000.
MeV2GeV = 1./1000.

def main():
    photon = False
    parser = argparse.ArgumentParser(description='Code to get root files from h5')
    parser.add_argument('-x', action="store", dest="xi", type=str, default="3")
    parser.add_argument('-g', action="store", dest="gamma", type=str, default="0.7")
    parser.add_argument('-v', action="store", dest="version", type=str, default="1")
    parser.add_argument('-i', action="store", dest="iteration", type=str, default="1")
    args         = parser.parse_args()
    xiInput      = args.xi
    gammaInput   = args.gamma
    ver          = args.version
    iter         = args.iteration
    
    ### for grid files
    inputDir  = "/storage/agrp/arkas/PtarmiganLUXEWorkAreaVertical/"
    # inputDir  = "/storage/agrp/arkas/PtarmiganWorkArea13GeVEBeam/"
    storage   = inputDir

    # p         = subprocess.Popen("mkdir -p "+storage, shell=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    # out, err  = p.communicate()

    ### for E320, grid, all particles
    tf = TFile( storage+'run_'+iter+'/raw_e320_xi'+str(xiInput)+'_gamma'+str(gammaInput)+'_v'+ver+'.root', 'recreate' )
    print("The output file: ", storage+'run_'+iter+'/raw_e320_xi'+str(xiInput)+'_gamma'+str(gammaInput)+'_v'+ver+'.root')

    ### for E320, grid, only positron
    # tf = TFile( storage+'run_'+iter+'/raw_e320_xi'+str(xiInput)+'_gamma'+str(gammaInput)+'_v'+ver+'_onlypositron.root', 'recreate' )
    # print("The output file: ", storage+'run_'+iter+'/raw_e320_xi'+str(xiInput)+'_gamma'+str(gammaInput)+'_v'+ver+'_onlypositron.root')
    

    tt_out    = TTree('tt','tt')
    vx_out    = ROOT.std.vector( float )()
    vy_out    = ROOT.std.vector( float )()
    vz_out    = ROOT.std.vector( float )()
    px_out    = ROOT.std.vector( float )()
    py_out    = ROOT.std.vector( float )()
    pz_out    = ROOT.std.vector( float )()
    eta_out   = ROOT.std.vector( float )()
    theta_out = ROOT.std.vector( float )()
    phi_out   = ROOT.std.vector( float )()
    E_out     = ROOT.std.vector( float )()
    pdgId_out = ROOT.std.vector( int )()
    mpid_out  = ROOT.std.vector( str )()
    wgt_out   = ROOT.std.vector( float )()
    time_out  = ROOT.std.vector( float )()
    xi_out    = ROOT.std.vector( float )()
    tt_out.Branch('vx', vx_out)
    tt_out.Branch('vy', vy_out)
    tt_out.Branch('vz', vz_out)
    tt_out.Branch('px', px_out)
    tt_out.Branch('py', py_out)
    tt_out.Branch('pz', pz_out)
    tt_out.Branch('eta', eta_out)
    tt_out.Branch('theta', theta_out)
    tt_out.Branch('phi', phi_out)
    tt_out.Branch('E',  E_out)
    tt_out.Branch('wgt',  wgt_out)
    tt_out.Branch('pdgId',pdgId_out)
    tt_out.Branch('mpid',mpid_out)
    tt_out.Branch('time',time_out)
    tt_out.Branch('xi',xi_out)

    
    ### for E320, grid
    print("The input h5 file directory: ", inputDir+"run_"+iter+"/*.dat")
    fInsCheck = glob.glob(inputDir+"run_"+iter+"/*.dat")
    if(len(fInsCheck)==0):
        print("No output here. Stopping")
        sys.exit()

    # fIns = [, 
    #         inputDir+"run_"+iter+"/positron_y.dat",
    #         inputDir+"run_"+iter+"/positron_z.dat",
    #         inputDir+"run_"+iter+"/positron_px.dat",
    #         inputDir+"run_"+iter+"/positron_py.dat",
    #         inputDir+"run_"+iter+"/positron_pz.dat",
    #         inputDir+"run_"+iter+"/positron_energy.dat"]
        
    xValue      = []
    yValue      = []
    zValue      = []
    pxValue     = []
    pyValue     = []
    pzValue     = []
    energyValue = []

    
    fInX = open(inputDir+"run_"+iter+"/positron_x.dat", 'r')
    print("reading: ",inputDir+"run_"+iter+"/positron_x.dat")

    lineX = 0
    for lines in fInX.readlines():
        lineX += 1
        eachWord = lines.split()
        print("x: ", eachWord[0])
        if lineX > 2:
            xValue.append(float(eachWord[0]))

    fInY = open(inputDir+"run_"+iter+"/positron_y.dat", 'r')
    print("reading: ",inputDir+"run_"+iter+"/positron_y.dat")
    lineY = 0
    for lines in fInY.readlines():
        lineY += 1
        eachWord = lines.split()
        if lineY > 2:
            yValue.append(float(eachWord[0]))

    fInZ = open(inputDir+"run_"+iter+"/positron_z.dat", 'r')
    print("reading: ",inputDir+"run_"+iter+"/positron_z.dat")
    lineZ = 0
    for lines in fInZ.readlines():
        lineZ += 1
        eachWord = lines.split()
        if lineZ > 2:
            zValue.append(float(eachWord[0]))

    fInPx = open(inputDir+"run_"+iter+"/positron_px.dat", 'r')
    print("reading: ",inputDir+"run_"+iter+"/positron_px.dat")
    linePx = 0
    for lines in fInPx.readlines():
        linePx += 1
        eachWord = lines.split()
        if linePx > 2:
            pxValue.append(float(eachWord[0]))

    fInPy = open(inputDir+"run_"+iter+"/positron_py.dat", 'r')
    print("reading: ",inputDir+"run_"+iter+"/positron_py.dat")
    linePy = 0
    for lines in fInPy.readlines():
        linePy += 1
        eachWord = lines.split()
        if linePy > 2:
            pyValue.append(float(eachWord[0]))

    fInPz = open(inputDir+"run_"+iter+"/positron_pz.dat", 'r')
    print("reading: ",inputDir+"run_"+iter+"/positron_pz.dat")
    linePz = 0
    for lines in fInPz.readlines():
        linePz += 1
        eachWord = lines.split()
        if linePz > 2:
            pzValue.append(float(eachWord[0]))

    fInE = open(inputDir+"run_"+iter+"/positron_energy.dat", 'r')
    print("reading: ",inputDir+"run_"+iter+"/positron_energy.dat")
    lineE = 0
    for lines in fInE.readlines():
        lineE += 1
        eachWord = lines.split()
        if lineE > 2:
            energyValue.append(float(eachWord[0]))

    # print(xValue)
    # print(yValue)
    # print(zValue)
    # print(pxValue)
    # print(pyValue)
    # print(pzValue)
    # print(energyValue)

    if(True):           
        ### photons
        photonNumber = 0
        if(False):
            id_value_photon       = fIn['final-state/photon']['id'][()]
            parentid_value_photon = fIn['final-state/photon']['parent_id'][()]
            momentum_value_photon = fIn['final-state/photon']['momentum'][()]
            position_value_photon = fIn['final-state/photon']['position'][()]
            weight_value_photon   = fIn['final-state/photon']['weight'][()]
            print("this file has ",len(id_value_photon)," photons")

            #### if(highestPhotonEvents>50):
            ####     if len(id_value_photon) < highestPhotonEvents/2.0: 
            ####         print("This file ",name," has very few photons ",len(id_value_photon), " ---- NOT PROCESSING")
            ####         continue
            #### else:
            ####     if len(id_value_photon) < highestPhotonEvents/10.0: 
            ####         print("This file ",name," has very few photons ",len(id_value_photon), " ---- NOT PROCESSING")
            ####         continue

            for j in range(0, len(id_value_photon)):
                if(photonNumber%10000==0): print("processed: ", photonNumber," photons")
                ### clear output tree branches
                mpid_out.clear()
                pdgId_out.clear()
                wgt_out.clear()
                vx_out.clear()
                vy_out.clear()
                vz_out.clear()
                px_out.clear()
                py_out.clear()
                pz_out.clear()
                eta_out.clear()
                theta_out.clear()
                phi_out.clear()
                E_out.clear()
                xi_out.clear()
                time_out.clear()

                vx0    = position_value_photon[j][0]*1.e2 ## m to cm
                vy0    = position_value_photon[j][1]*1.e2 ## m to cm
                vz0    = position_value_photon[j][2]*1.e2 ## m to cm
                t0     = position_value_photon[j][3]
                Energy = momentum_value_photon[j][0]*1.e-3 ## MeV to GeV
                px0    = momentum_value_photon[j][1]*1.e-3 ## MeV to GeV
                py0    = momentum_value_photon[j][2]*1.e-3 ## MeV to GeV
                pz0    = momentum_value_photon[j][3]*1.e-3 ## MeV to GeV

                photonVec = TLorentzVector() 
                photonVec.SetPxPyPzE(px0, py0, pz0, Energy)

                eta0   = photonVec.Eta()
                theta0 = photonVec.Theta()
                phi0   = photonVec.Phi()

                pdgId0 = 22
                wgt0   = weight_value_photon[j]
                MP_ID  = str(id_value_photon[j])+"_"+str(pdgId0)
                xi0    = float(xiInput)
                mpid_out.push_back(str(MP_ID))
                wgt_out.push_back(wgt0)  
                pdgId_out.push_back(int(pdgId0))  
                vx_out.push_back(vx0)
                vy_out.push_back(vy0)
                vz_out.push_back(vz0)
                px_out.push_back(px0)
                py_out.push_back(py0)
                pz_out.push_back(pz0)
                eta_out.push_back(eta0)
                theta_out.push_back(theta0)
                phi_out.push_back(phi0)
                E_out.push_back(Energy)
                time_out.push_back(t0)
                xi_out.push_back(xi0)
                photonNumber += 1
                tt_out.Fill()


        electronNumber = 0
        if(False):
            ### electrons are only collected for g+laser
            id_value_electron       = fIn['final-state/electron']['id'][()]
            print("this file has ",len(id_value_electron)," electrons")
            parentid_value_electron = fIn['final-state/electron']['parent_id'][()]
            momentum_value_electron = fIn['final-state/electron']['momentum'][()]
            position_value_electron = fIn['final-state/electron']['position'][()]
            weight_value_electron   = fIn['final-state/electron']['weight'][()]
            for j in range(0, len(id_value_electron)):
                if(electronNumber%10000==0): print("processed: ", electronNumber," electrons")
                mpid_out.clear()
                pdgId_out.clear()
                wgt_out.clear()
                vx_out.clear()
                vy_out.clear()
                vz_out.clear()
                px_out.clear()
                py_out.clear()
                pz_out.clear()
                eta_out.clear()
                theta_out.clear()
                phi_out.clear()
                E_out.clear()
                xi_out.clear()
                time_out.clear()

                vx0    = position_value_electron[j][0]*1.e2 ## m to cm
                vy0    = position_value_electron[j][1]*1.e2 ## m to cm
                vz0    = position_value_electron[j][2]*1.e2 ## m to cm
                t0     = position_value_electron[j][3]
                Energy = momentum_value_electron[j][0]*1.e-3 ## MeV to GeV
                px0    = momentum_value_electron[j][1]*1.e-3 ## MeV to GeV
                py0    = momentum_value_electron[j][2]*1.e-3 ## MeV to GeV
                pz0    = momentum_value_electron[j][3]*1.e-3 ## MeV to GeV

                electronVec = TLorentzVector()
                electronVec.SetPxPyPzE(px0, py0, pz0, Energy)

                eta0   = electronVec.Eta()
                theta0 = electronVec.Theta()
                phi0   = electronVec.Phi()


                pdgId0 = 11
                wgt0   = weight_value_electron[j]
                MP_ID  = str(id_value_electron[j])+"_"+str(pdgId0)
                xi0    = float(xiInput)
                mpid_out.push_back(str(MP_ID))
                wgt_out.push_back(wgt0)  
                pdgId_out.push_back(int(pdgId0))  
                vx_out.push_back(vx0)
                vy_out.push_back(vy0)
                vz_out.push_back(vz0)
                px_out.push_back(px0)
                py_out.push_back(py0)
                pz_out.push_back(pz0)
                eta_out.push_back(eta0)
                theta_out.push_back(theta0)
                phi_out.push_back(phi0)
                E_out.push_back(Energy)
                time_out.push_back(t0)
                xi_out.push_back(xi0)
                electronNumber += 1
                tt_out.Fill()
        

        positronNumber = 0
        if(True):
            for j in range(0, len(xValue)):
                if(positronNumber%1==0): print("processed: ", positronNumber," positrons")
                
                mpid_out.clear()
                pdgId_out.clear()
                wgt_out.clear()
                vx_out.clear()
                vy_out.clear()
                vz_out.clear()
                px_out.clear()
                py_out.clear()
                pz_out.clear()
                eta_out.clear()
                theta_out.clear()
                phi_out.clear()
                E_out.clear()
                xi_out.clear()
                time_out.clear()

                vx0    = xValue[j]*1.e2 ## m to cm
                vy0    = yValue[j]*1.e2 ## m to cm
                vz0    = zValue[j]*1.e2 ## m to cm
                t0     = 0.0
                Energy = energyValue[j]*1.e-3 ## MeV to GeV
                px0    = pxValue[j]*1.e-3 ## MeV to GeV
                py0    = pyValue[j]*1.e-3 ## MeV to GeV
                pz0    = pzValue[j]*1.e-3 ## MeV to GeV

                positronVec = TLorentzVector()
                positronVec.SetPxPyPzE(px0, py0, pz0, Energy)

                eta0   = positronVec.Eta()
                theta0 = positronVec.Theta()
                phi0   = positronVec.Phi()


                pdgId0 = -11
                #### adding ad hoc for now
                wgt0   = 1.0
                MP_ID  = str("1_"+str(pdgId0))
                xi0    = float(xiInput)
                mpid_out.push_back(str(MP_ID))
                wgt_out.push_back(wgt0)  
                pdgId_out.push_back(int(pdgId0))  
                vx_out.push_back(vx0)
                vy_out.push_back(vy0)
                vz_out.push_back(vz0)
                px_out.push_back(px0)
                py_out.push_back(py0)
                pz_out.push_back(pz0)
                eta_out.push_back(eta0)
                theta_out.push_back(theta0)
                phi_out.push_back(phi0)
                E_out.push_back(Energy)
                time_out.push_back(t0)
                xi_out.push_back(xi0)
                positronNumber += 1
                tt_out.Fill()
            

        
        print("electrons ", electronNumber, " photons ", photonNumber, " and positrons ", positronNumber, " in file ")
        
    tt_out.Write()
    tf.Write()
    tf.Write()
    tf.Close()


if __name__=="__main__":
    intime = time.time()
    main()
    print("----- the time taken ", time.time() - intime, " s")
