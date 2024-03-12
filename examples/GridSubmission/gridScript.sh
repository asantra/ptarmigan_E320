#! /bin/bash
#PBS -m n
#PBS -l walltime=24:00:00

#### script that run the python script, the MadGraph generator
echo "Installing Root>>>>>>"
export ATLAS_LOCAL_ROOT_BASE=/cvmfs/atlas.cern.ch/repo/ATLASLocalRootBase
source ${ATLAS_LOCAL_ROOT_BASE}/user/atlasLocalSetup.sh
### for latest root
lsetup "views LCG_104b_ATLAS_2 x86_64-centos7-gcc11-opt"

### linking HDF5 library
export LD_LIBRARY_PATH=/srv01/agrp/arkas/HDF5/HDF5_Install/lib:${LD_LIBRARY_PATH}
echo "running the Ptarmigan script>>>>>>"
iteration=${parname1}
directory=${parname2}
outLoc=${parname3}

outDir=run_${iteration}
outFinalLoc=${outLoc}/${outDir}



#### go to the directory where the files live
cd ${directory}
echo "I am now in "${PWD}
echo "prepare output directory"
mkdir -p ${outFinalLoc}
#### for E320
# cp E320_profile_MASTER.yml ${outFinalLoc}/E320_profile_Iteration${iteration}.conf
# sed -i -e "s|RNDMSEED|${iteration}|g" ${outFinalLoc}/E320_profile_Iteration${iteration}.conf
# # # time /srv01/agrp/arkas/Light_By_Light_Scattering/ptarmigan/./target/release/ptarmigan ${outFinalLoc}/E320_profile_Iteration${iteration}.conf
# time /srv01/agrp/arkas/Light_By_Light_Scattering/Vertical_Angle/ptarmigan/./target/release/ptarmigan ${outFinalLoc}/E320_profile_Iteration${iteration}.conf

### for LUXE
cp luxe_tdr_MASTER.yml ${outFinalLoc}/luxe_tdr_Iteration${iteration}.conf
sed -i -e "s|RNDMSEED|${iteration}|g" ${outFinalLoc}/luxe_tdr_Iteration${iteration}.conf

time /srv01/agrp/arkas/Light_By_Light_Scattering/ptarmigan/./target/release/ptarmigan ${outFinalLoc}/luxe_tdr_Iteration${iteration}.conf
