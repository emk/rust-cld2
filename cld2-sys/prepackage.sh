#!/bin/sh
cd cld2
git checkout .
rm -rf docs/
for i in internal/cld_generated_*.cc internal/cld2_generated_*.cc; do
	# NOTE: make sure that this whitelist reflects the file list in src/build.rs!
	case $i in
	*/cld_generated_cjk_uni_prop_80.cc|\
	*/cld2_generated_cjk_compatible.cc|\
	*/cld_generated_cjk_delta_bi_32.cc|\
	*/cld2_generated_quad0122.cc|\
	*/cld2_generated_deltaocta0122.cc|\
	*/cld2_generated_distinctocta0122.cc|\
	*/cld_generated_score_quad_octa_0122.cc)
		echo $i: processed
		sed -i 's/^\([ \t]*{*\(0x[0-9a-fA-F]\+,[ \t]*\)\{3\}0x[0-9a-fA-F]\+}*,\)[ \t]*\/\/.*$/\1/' $i
		;;
	*)
		echo $i: removed
		rm -f $i
		;;
	esac
done
